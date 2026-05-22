import lldb, socket, re, time

DIRECTMAP_BASE = 0xffff800000000000
ALLOC_RET = 0xffffffff80053b88   # allocate_kernel_stack return site in spawn_verified
MON = ("127.0.0.1", 55557)
_armed = [False]

def gva2gpa(gva):
    try:
        s = socket.create_connection(MON, timeout=3)
        time.sleep(0.2); s.recv(8192)
        s.sendall(("gva2gpa 0x%x\n" % gva).encode())
        time.sleep(0.4)
        resp = s.recv(16384).decode(errors="ignore")
        s.close()
        m = re.search(r"gpa:\s*0x([0-9a-fA-F]+)", resp)
        return int(m.group(1), 16) if m else None
    except Exception as e:
        print("mon err %s" % e); return None

def in_kstack_region(v):
    return 0xffffff5000000000 <= v <= 0xffffff5fffffffff

def on_proof_io(frame, bp_loc, internal_dict):
    if _armed[0]:
        return False
    target = frame.GetThread().GetProcess().GetTarget()
    bp = target.BreakpointCreateByAddress(ALLOC_RET)
    bp.SetScriptCallbackFunction("wp2.on_alloc_return")
    bp.SetOneShot(True)
    print("=== entered spawn_proof_io_capsule; watching its kstack alloc ===")
    return False

def on_alloc_return(frame, bp_loc, internal_dict):
    rdx = frame.FindRegister("rdx").GetValueAsUnsigned()
    rax = frame.FindRegister("rax").GetValueAsUnsigned()
    print("=== alloc return rax=0x%x rdx=0x%x ===" % (rax, rdx))
    top = rdx if in_kstack_region(rdx) else (rax if in_kstack_region(rax) else 0)
    if top == 0:
        return False
    slot = top - 24
    phys = gva2gpa(slot)
    if phys is None:
        print("=== kstack_top=0x%x slot=0x%x gva2gpa failed ===" % (top, slot))
        return False
    dm = DIRECTMAP_BASE + phys
    we = lldb.SBError()
    frame.GetThread().GetProcess().GetTarget().WatchAddress(dm, 8, False, True, we)
    _armed[0] = True
    print("=== ARMED: kstack_top=0x%x slot=0x%x phys=0x%x directmap=0x%x wp=%s ===" %
          (top, slot, phys, dm, we.GetCString()))
    return False
