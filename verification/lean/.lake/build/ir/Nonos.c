// Lean compiler output
// Module: Nonos
// Imports: Init Nonos.AntiRollback Nonos.Authorization Nonos.Capability Nonos.CapabilityBits Nonos.Crypto Nonos.Ipc Nonos.Isolation Nonos.Path Nonos.Paging
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_AntiRollback(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Authorization(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Capability(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_CapabilityBits(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Crypto(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Ipc(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Isolation(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Path(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Paging(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Nonos(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_AntiRollback(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Authorization(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Capability(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_CapabilityBits(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Crypto(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Ipc(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Isolation(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Path(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Paging(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
