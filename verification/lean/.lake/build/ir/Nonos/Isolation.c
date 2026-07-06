// Lean compiler output
// Module: Nonos.Isolation
// Imports: Init
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
LEAN_EXPORT lean_object* l_Nonos_Isolation_pteWritable___boxed(lean_object*);
LEAN_EXPORT uint8_t l_Nonos_Isolation_pteExecutable(lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Isolation_userEnd;
LEAN_EXPORT lean_object* l_Nonos_Isolation_maxCopy;
LEAN_EXPORT uint8_t l_Nonos_Isolation_pteWritable(lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Isolation_pteExecutable___boxed(lean_object*);
LEAN_EXPORT uint8_t l_Nonos_Isolation_pteWritable(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = lean_ctor_get_uint8(x_1, 0);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Nonos_Isolation_pteWritable___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = l_Nonos_Isolation_pteWritable(x_1);
lean_dec(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT uint8_t l_Nonos_Isolation_pteExecutable(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = lean_ctor_get_uint8(x_1, 1);
return x_2;
}
}
LEAN_EXPORT lean_object* l_Nonos_Isolation_pteExecutable___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = l_Nonos_Isolation_pteExecutable(x_1);
lean_dec(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
static lean_object* _init_l_Nonos_Isolation_userEnd() {
_start:
{
lean_object* x_1; 
x_1 = lean_cstr_to_nat("140737488355327");
return x_1;
}
}
static lean_object* _init_l_Nonos_Isolation_maxCopy() {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(67108864u);
return x_1;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Nonos_Isolation(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Nonos_Isolation_userEnd = _init_l_Nonos_Isolation_userEnd();
lean_mark_persistent(l_Nonos_Isolation_userEnd);
l_Nonos_Isolation_maxCopy = _init_l_Nonos_Isolation_maxCopy();
lean_mark_persistent(l_Nonos_Isolation_maxCopy);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
