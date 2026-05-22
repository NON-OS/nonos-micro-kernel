import lldb, socket, re, time

MON = ("127.0.0.1", 55557)
state = {"slot": 0, "phys_pre": 0}

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

def on_proof_io(frame, bp_loc, internal_dict):
    if state["slot"]:
        return False
    target = frame.GetThread().GetProcess().GetTarget()
    bp = target.BreakpointCreateByAddress(0xffffffff80036abb)
    bp.SetScriptCallbackFunction("wp3.on_memset")
    bp.SetOneShot(True)
    return False

def on_memset(frame, bp_loc, internal_dict):
    base = frame.FindRegister("rdi").GetValueAsUnsigned()
    slot = base + 0x4000 - 24
    phys = gva2gpa(slot)
    state["slot"] = slot
    state["phys_pre"] = phys or 0
    print("=== PRE: slot=0x%x phys=0x%x ===" % (slot, phys or 0))
    return False

def on_crash(frame, bp_loc, internal_dict):
    slot = state["slot"]
    if not slot:
        return True
    phys_post = gva2gpa(slot)
    process = frame.GetThread().GetProcess()
    err = lldb.SBError()
    data = process.ReadMemory(slot, 8, err)
    sv = int.from_bytes(bytes(data), "little") if err.Success() else -1
    print("=== CRASH: slot=0x%x [slot]=0x%x phys_pre=0x%x phys_post=0x%x ===" %
          (slot, sv & 0xffffffffffffffff, state["phys_pre"], phys_post or 0))
    return True
