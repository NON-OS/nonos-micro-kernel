// Lean compiler output
// Module: Nonos.Secure
// Imports: Init Nonos.Capability Nonos.Isolation Nonos.AntiRollback
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
LEAN_EXPORT lean_object* l___private_Nonos_Secure_0__Nonos_Secure_step_match__1_splitter(lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__8(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__2___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__2(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__6(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__12(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__11(lean_object*, lean_object*);
uint8_t l_instDecidableNot___rarg(uint8_t);
LEAN_EXPORT lean_object* l_Nonos_Secure_run(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__9(lean_object*, lean_object*, lean_object*, lean_object*);
extern lean_object* l_Nonos_Isolation_userEnd;
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__4___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
extern lean_object* l_Nonos_Isolation_maxCopy;
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__5(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__6___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_Nonos_Capability_attenuate(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__4(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
uint8_t l_Nonos_Capability_grant(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Nonos_Secure_0__Nonos_Secure_step_match__1_splitter___rarg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__7(lean_object*, lean_object*, lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__13(lean_object*, lean_object*);
static lean_object* l_Nonos_Secure_step___closed__1;
lean_object* l_Nonos_AntiRollback_update(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__7___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__3(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__10(lean_object*, lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step(lean_object*, lean_object*);
uint8_t l_Nonos_Capability_revoke(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__9___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__14(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__1(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
lean_dec(x_1);
x_4 = lean_apply_1(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__2(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; 
x_6 = lean_nat_dec_eq(x_4, x_2);
if (x_6 == 0)
{
lean_object* x_7; lean_object* x_8; 
lean_dec(x_3);
x_7 = lean_ctor_get(x_1, 0);
lean_inc(x_7);
lean_dec(x_1);
x_8 = lean_apply_2(x_7, x_4, x_5);
return x_8;
}
else
{
lean_object* x_9; lean_object* x_10; lean_object* x_11; 
x_9 = lean_ctor_get(x_1, 0);
lean_inc(x_9);
lean_dec(x_1);
x_10 = lean_apply_1(x_9, x_4);
x_11 = l_Nonos_Capability_attenuate(x_10, x_3, x_5);
return x_11;
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__3(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
lean_dec(x_1);
x_4 = lean_apply_1(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__4(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; 
x_6 = lean_nat_dec_eq(x_4, x_1);
if (x_6 == 0)
{
lean_object* x_7; 
x_7 = lean_apply_2(x_3, x_4, x_5);
return x_7;
}
else
{
lean_object* x_8; uint8_t x_9; lean_object* x_10; 
x_8 = lean_apply_1(x_3, x_4);
x_9 = l_Nonos_Capability_grant(x_8, x_2, x_5);
x_10 = lean_box(x_9);
return x_10;
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__5(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
lean_dec(x_1);
x_4 = lean_apply_1(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__6(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; 
x_6 = lean_nat_dec_eq(x_4, x_2);
if (x_6 == 0)
{
lean_object* x_7; lean_object* x_8; 
x_7 = lean_ctor_get(x_1, 0);
lean_inc(x_7);
lean_dec(x_1);
x_8 = lean_apply_2(x_7, x_4, x_5);
return x_8;
}
else
{
lean_object* x_9; lean_object* x_10; uint8_t x_11; lean_object* x_12; 
x_9 = lean_ctor_get(x_1, 0);
lean_inc(x_9);
lean_dec(x_1);
x_10 = lean_apply_1(x_9, x_4);
x_11 = l_Nonos_Capability_revoke(x_10, x_3, x_5);
x_12 = lean_box(x_11);
return x_12;
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__7(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_eq(x_4, x_2);
if (x_5 == 0)
{
lean_object* x_6; lean_object* x_7; 
lean_dec(x_3);
x_6 = lean_ctor_get(x_1, 1);
lean_inc(x_6);
lean_dec(x_1);
x_7 = lean_apply_1(x_6, x_4);
return x_7;
}
else
{
lean_object* x_8; 
lean_dec(x_4);
lean_dec(x_1);
x_8 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_8, 0, x_3);
return x_8;
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__8(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
lean_dec(x_1);
x_5 = lean_apply_2(x_4, x_2, x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__9(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_eq(x_4, x_2);
if (x_5 == 0)
{
lean_object* x_6; lean_object* x_7; 
lean_dec(x_3);
x_6 = lean_ctor_get(x_1, 1);
lean_inc(x_6);
lean_dec(x_1);
x_7 = lean_apply_1(x_6, x_4);
return x_7;
}
else
{
lean_object* x_8; 
lean_dec(x_4);
lean_dec(x_1);
x_8 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_8, 0, x_3);
return x_8;
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__10(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
lean_dec(x_1);
x_5 = lean_apply_2(x_4, x_2, x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__11(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
lean_dec(x_1);
x_4 = lean_apply_1(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__12(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
lean_dec(x_1);
x_5 = lean_apply_2(x_4, x_2, x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__13(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; 
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
lean_dec(x_1);
x_4 = lean_apply_1(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__14(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; lean_object* x_5; 
x_4 = lean_ctor_get(x_1, 0);
lean_inc(x_4);
lean_dec(x_1);
x_5 = lean_apply_2(x_4, x_2, x_3);
return x_5;
}
}
static lean_object* _init_l_Nonos_Secure_step___closed__1() {
_start:
{
lean_object* x_1; lean_object* x_2; lean_object* x_3; 
x_1 = l_Nonos_Isolation_userEnd;
x_2 = lean_unsigned_to_nat(1u);
x_3 = lean_nat_add(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step(lean_object* x_1, lean_object* x_2) {
_start:
{
switch (lean_obj_tag(x_2)) {
case 0:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; uint8_t x_7; 
x_3 = lean_ctor_get(x_2, 0);
lean_inc(x_3);
x_4 = lean_ctor_get(x_2, 1);
lean_inc(x_4);
lean_dec(x_2);
lean_inc(x_1);
x_5 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__2___boxed), 5, 3);
lean_closure_set(x_5, 0, x_1);
lean_closure_set(x_5, 1, x_3);
lean_closure_set(x_5, 2, x_4);
lean_inc(x_1);
x_6 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__1), 2, 1);
lean_closure_set(x_6, 0, x_1);
x_7 = !lean_is_exclusive(x_1);
if (x_7 == 0)
{
lean_object* x_8; lean_object* x_9; 
x_8 = lean_ctor_get(x_1, 1);
lean_dec(x_8);
x_9 = lean_ctor_get(x_1, 0);
lean_dec(x_9);
lean_ctor_set(x_1, 1, x_6);
lean_ctor_set(x_1, 0, x_5);
return x_1;
}
else
{
lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_10 = lean_ctor_get(x_1, 2);
x_11 = lean_ctor_get(x_1, 3);
lean_inc(x_11);
lean_inc(x_10);
lean_dec(x_1);
x_12 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_12, 0, x_5);
lean_ctor_set(x_12, 1, x_6);
lean_ctor_set(x_12, 2, x_10);
lean_ctor_set(x_12, 3, x_11);
return x_12;
}
}
case 1:
{
lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; uint8_t x_20; 
x_13 = lean_ctor_get(x_2, 0);
lean_inc(x_13);
x_14 = lean_ctor_get(x_2, 1);
lean_inc(x_14);
x_15 = lean_ctor_get(x_2, 2);
lean_inc(x_15);
lean_dec(x_2);
x_16 = lean_ctor_get(x_1, 0);
lean_inc(x_16);
x_17 = lean_ctor_get(x_1, 2);
lean_inc(x_17);
x_18 = lean_ctor_get(x_1, 3);
lean_inc(x_18);
lean_inc(x_16);
lean_inc(x_15);
x_19 = lean_apply_2(x_16, x_13, x_15);
x_20 = lean_unbox(x_19);
lean_dec(x_19);
if (x_20 == 0)
{
lean_dec(x_18);
lean_dec(x_17);
lean_dec(x_16);
lean_dec(x_15);
lean_dec(x_14);
return x_1;
}
else
{
lean_object* x_21; lean_object* x_22; uint8_t x_23; 
x_21 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__4___boxed), 5, 3);
lean_closure_set(x_21, 0, x_14);
lean_closure_set(x_21, 1, x_15);
lean_closure_set(x_21, 2, x_16);
lean_inc(x_1);
x_22 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__3), 2, 1);
lean_closure_set(x_22, 0, x_1);
x_23 = !lean_is_exclusive(x_1);
if (x_23 == 0)
{
lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; 
x_24 = lean_ctor_get(x_1, 3);
lean_dec(x_24);
x_25 = lean_ctor_get(x_1, 2);
lean_dec(x_25);
x_26 = lean_ctor_get(x_1, 1);
lean_dec(x_26);
x_27 = lean_ctor_get(x_1, 0);
lean_dec(x_27);
lean_ctor_set(x_1, 1, x_22);
lean_ctor_set(x_1, 0, x_21);
return x_1;
}
else
{
lean_object* x_28; 
lean_dec(x_1);
x_28 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_28, 0, x_21);
lean_ctor_set(x_28, 1, x_22);
lean_ctor_set(x_28, 2, x_17);
lean_ctor_set(x_28, 3, x_18);
return x_28;
}
}
}
case 2:
{
lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; uint8_t x_33; 
x_29 = lean_ctor_get(x_2, 0);
lean_inc(x_29);
x_30 = lean_ctor_get(x_2, 1);
lean_inc(x_30);
lean_dec(x_2);
lean_inc(x_1);
x_31 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__6___boxed), 5, 3);
lean_closure_set(x_31, 0, x_1);
lean_closure_set(x_31, 1, x_29);
lean_closure_set(x_31, 2, x_30);
lean_inc(x_1);
x_32 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__5), 2, 1);
lean_closure_set(x_32, 0, x_1);
x_33 = !lean_is_exclusive(x_1);
if (x_33 == 0)
{
lean_object* x_34; lean_object* x_35; 
x_34 = lean_ctor_get(x_1, 1);
lean_dec(x_34);
x_35 = lean_ctor_get(x_1, 0);
lean_dec(x_35);
lean_ctor_set(x_1, 1, x_32);
lean_ctor_set(x_1, 0, x_31);
return x_1;
}
else
{
lean_object* x_36; lean_object* x_37; lean_object* x_38; 
x_36 = lean_ctor_get(x_1, 2);
x_37 = lean_ctor_get(x_1, 3);
lean_inc(x_37);
lean_inc(x_36);
lean_dec(x_1);
x_38 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_38, 0, x_31);
lean_ctor_set(x_38, 1, x_32);
lean_ctor_set(x_38, 2, x_36);
lean_ctor_set(x_38, 3, x_37);
return x_38;
}
}
case 3:
{
lean_object* x_39; uint8_t x_40; 
x_39 = lean_ctor_get(x_2, 1);
lean_inc(x_39);
x_40 = lean_ctor_get_uint8(x_39, 0);
if (x_40 == 0)
{
lean_object* x_41; lean_object* x_42; lean_object* x_43; uint8_t x_44; 
x_41 = lean_ctor_get(x_2, 0);
lean_inc(x_41);
lean_dec(x_2);
lean_inc(x_1);
x_42 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__8), 3, 1);
lean_closure_set(x_42, 0, x_1);
lean_inc(x_1);
x_43 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__7___boxed), 4, 3);
lean_closure_set(x_43, 0, x_1);
lean_closure_set(x_43, 1, x_41);
lean_closure_set(x_43, 2, x_39);
x_44 = !lean_is_exclusive(x_1);
if (x_44 == 0)
{
lean_object* x_45; lean_object* x_46; 
x_45 = lean_ctor_get(x_1, 1);
lean_dec(x_45);
x_46 = lean_ctor_get(x_1, 0);
lean_dec(x_46);
lean_ctor_set(x_1, 1, x_43);
lean_ctor_set(x_1, 0, x_42);
return x_1;
}
else
{
lean_object* x_47; lean_object* x_48; lean_object* x_49; 
x_47 = lean_ctor_get(x_1, 2);
x_48 = lean_ctor_get(x_1, 3);
lean_inc(x_48);
lean_inc(x_47);
lean_dec(x_1);
x_49 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_49, 0, x_42);
lean_ctor_set(x_49, 1, x_43);
lean_ctor_set(x_49, 2, x_47);
lean_ctor_set(x_49, 3, x_48);
return x_49;
}
}
else
{
uint8_t x_50; 
x_50 = lean_ctor_get_uint8(x_39, 1);
if (x_50 == 0)
{
lean_object* x_51; lean_object* x_52; lean_object* x_53; uint8_t x_54; 
x_51 = lean_ctor_get(x_2, 0);
lean_inc(x_51);
lean_dec(x_2);
lean_inc(x_1);
x_52 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__10), 3, 1);
lean_closure_set(x_52, 0, x_1);
lean_inc(x_1);
x_53 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__9___boxed), 4, 3);
lean_closure_set(x_53, 0, x_1);
lean_closure_set(x_53, 1, x_51);
lean_closure_set(x_53, 2, x_39);
x_54 = !lean_is_exclusive(x_1);
if (x_54 == 0)
{
lean_object* x_55; lean_object* x_56; 
x_55 = lean_ctor_get(x_1, 1);
lean_dec(x_55);
x_56 = lean_ctor_get(x_1, 0);
lean_dec(x_56);
lean_ctor_set(x_1, 1, x_53);
lean_ctor_set(x_1, 0, x_52);
return x_1;
}
else
{
lean_object* x_57; lean_object* x_58; lean_object* x_59; 
x_57 = lean_ctor_get(x_1, 2);
x_58 = lean_ctor_get(x_1, 3);
lean_inc(x_58);
lean_inc(x_57);
lean_dec(x_1);
x_59 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_59, 0, x_52);
lean_ctor_set(x_59, 1, x_53);
lean_ctor_set(x_59, 2, x_57);
lean_ctor_set(x_59, 3, x_58);
return x_59;
}
}
else
{
lean_dec(x_39);
lean_dec(x_2);
return x_1;
}
}
}
case 4:
{
uint8_t x_60; 
x_60 = !lean_is_exclusive(x_2);
if (x_60 == 0)
{
lean_object* x_61; lean_object* x_62; lean_object* x_63; uint8_t x_64; uint8_t x_65; 
x_61 = lean_ctor_get(x_2, 0);
x_62 = lean_ctor_get(x_2, 1);
x_63 = lean_unsigned_to_nat(0u);
x_64 = lean_nat_dec_eq(x_61, x_63);
x_65 = l_instDecidableNot___rarg(x_64);
if (x_65 == 0)
{
lean_free_object(x_2);
lean_dec(x_62);
lean_dec(x_61);
return x_1;
}
else
{
lean_object* x_66; uint8_t x_67; 
x_66 = l_Nonos_Isolation_maxCopy;
x_67 = lean_nat_dec_le(x_62, x_66);
if (x_67 == 0)
{
lean_free_object(x_2);
lean_dec(x_62);
lean_dec(x_61);
return x_1;
}
else
{
lean_object* x_68; lean_object* x_69; uint8_t x_70; 
x_68 = lean_nat_add(x_61, x_62);
x_69 = l_Nonos_Secure_step___closed__1;
x_70 = lean_nat_dec_le(x_68, x_69);
lean_dec(x_68);
if (x_70 == 0)
{
lean_free_object(x_2);
lean_dec(x_62);
lean_dec(x_61);
return x_1;
}
else
{
lean_object* x_71; lean_object* x_72; uint8_t x_73; 
lean_inc(x_1);
x_71 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__12), 3, 1);
lean_closure_set(x_71, 0, x_1);
lean_inc(x_1);
x_72 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__11), 2, 1);
lean_closure_set(x_72, 0, x_1);
lean_ctor_set_tag(x_2, 0);
x_73 = !lean_is_exclusive(x_1);
if (x_73 == 0)
{
lean_object* x_74; lean_object* x_75; lean_object* x_76; lean_object* x_77; 
x_74 = lean_ctor_get(x_1, 2);
x_75 = lean_ctor_get(x_1, 1);
lean_dec(x_75);
x_76 = lean_ctor_get(x_1, 0);
lean_dec(x_76);
x_77 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_77, 0, x_2);
lean_ctor_set(x_77, 1, x_74);
lean_ctor_set(x_1, 2, x_77);
lean_ctor_set(x_1, 1, x_72);
lean_ctor_set(x_1, 0, x_71);
return x_1;
}
else
{
lean_object* x_78; lean_object* x_79; lean_object* x_80; lean_object* x_81; 
x_78 = lean_ctor_get(x_1, 2);
x_79 = lean_ctor_get(x_1, 3);
lean_inc(x_79);
lean_inc(x_78);
lean_dec(x_1);
x_80 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_80, 0, x_2);
lean_ctor_set(x_80, 1, x_78);
x_81 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_81, 0, x_71);
lean_ctor_set(x_81, 1, x_72);
lean_ctor_set(x_81, 2, x_80);
lean_ctor_set(x_81, 3, x_79);
return x_81;
}
}
}
}
}
else
{
lean_object* x_82; lean_object* x_83; lean_object* x_84; uint8_t x_85; uint8_t x_86; 
x_82 = lean_ctor_get(x_2, 0);
x_83 = lean_ctor_get(x_2, 1);
lean_inc(x_83);
lean_inc(x_82);
lean_dec(x_2);
x_84 = lean_unsigned_to_nat(0u);
x_85 = lean_nat_dec_eq(x_82, x_84);
x_86 = l_instDecidableNot___rarg(x_85);
if (x_86 == 0)
{
lean_dec(x_83);
lean_dec(x_82);
return x_1;
}
else
{
lean_object* x_87; uint8_t x_88; 
x_87 = l_Nonos_Isolation_maxCopy;
x_88 = lean_nat_dec_le(x_83, x_87);
if (x_88 == 0)
{
lean_dec(x_83);
lean_dec(x_82);
return x_1;
}
else
{
lean_object* x_89; lean_object* x_90; uint8_t x_91; 
x_89 = lean_nat_add(x_82, x_83);
x_90 = l_Nonos_Secure_step___closed__1;
x_91 = lean_nat_dec_le(x_89, x_90);
lean_dec(x_89);
if (x_91 == 0)
{
lean_dec(x_83);
lean_dec(x_82);
return x_1;
}
else
{
lean_object* x_92; lean_object* x_93; lean_object* x_94; lean_object* x_95; lean_object* x_96; lean_object* x_97; lean_object* x_98; lean_object* x_99; 
lean_inc(x_1);
x_92 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__12), 3, 1);
lean_closure_set(x_92, 0, x_1);
lean_inc(x_1);
x_93 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__11), 2, 1);
lean_closure_set(x_93, 0, x_1);
x_94 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_94, 0, x_82);
lean_ctor_set(x_94, 1, x_83);
x_95 = lean_ctor_get(x_1, 2);
lean_inc(x_95);
x_96 = lean_ctor_get(x_1, 3);
lean_inc(x_96);
if (lean_is_exclusive(x_1)) {
 lean_ctor_release(x_1, 0);
 lean_ctor_release(x_1, 1);
 lean_ctor_release(x_1, 2);
 lean_ctor_release(x_1, 3);
 x_97 = x_1;
} else {
 lean_dec_ref(x_1);
 x_97 = lean_box(0);
}
x_98 = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(x_98, 0, x_94);
lean_ctor_set(x_98, 1, x_95);
if (lean_is_scalar(x_97)) {
 x_99 = lean_alloc_ctor(0, 4, 0);
} else {
 x_99 = x_97;
}
lean_ctor_set(x_99, 0, x_92);
lean_ctor_set(x_99, 1, x_93);
lean_ctor_set(x_99, 2, x_98);
lean_ctor_set(x_99, 3, x_96);
return x_99;
}
}
}
}
}
default: 
{
lean_object* x_100; lean_object* x_101; lean_object* x_102; uint8_t x_103; 
x_100 = lean_ctor_get(x_2, 0);
lean_inc(x_100);
lean_dec(x_2);
lean_inc(x_1);
x_101 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__14), 3, 1);
lean_closure_set(x_101, 0, x_1);
lean_inc(x_1);
x_102 = lean_alloc_closure((void*)(l_Nonos_Secure_step___elambda__13), 2, 1);
lean_closure_set(x_102, 0, x_1);
x_103 = !lean_is_exclusive(x_1);
if (x_103 == 0)
{
lean_object* x_104; lean_object* x_105; lean_object* x_106; lean_object* x_107; 
x_104 = lean_ctor_get(x_1, 3);
x_105 = lean_ctor_get(x_1, 1);
lean_dec(x_105);
x_106 = lean_ctor_get(x_1, 0);
lean_dec(x_106);
x_107 = l_Nonos_AntiRollback_update(x_104, x_100);
lean_dec(x_100);
lean_dec(x_104);
lean_ctor_set(x_1, 3, x_107);
lean_ctor_set(x_1, 1, x_102);
lean_ctor_set(x_1, 0, x_101);
return x_1;
}
else
{
lean_object* x_108; lean_object* x_109; lean_object* x_110; lean_object* x_111; 
x_108 = lean_ctor_get(x_1, 2);
x_109 = lean_ctor_get(x_1, 3);
lean_inc(x_109);
lean_inc(x_108);
lean_dec(x_1);
x_110 = l_Nonos_AntiRollback_update(x_109, x_100);
lean_dec(x_100);
lean_dec(x_109);
x_111 = lean_alloc_ctor(0, 4, 0);
lean_ctor_set(x_111, 0, x_101);
lean_ctor_set(x_111, 1, x_102);
lean_ctor_set(x_111, 2, x_108);
lean_ctor_set(x_111, 3, x_110);
return x_111;
}
}
}
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__2___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = l_Nonos_Secure_step___elambda__2(x_1, x_2, x_3, x_4, x_5);
lean_dec(x_2);
return x_6;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__4___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = l_Nonos_Secure_step___elambda__4(x_1, x_2, x_3, x_4, x_5);
lean_dec(x_2);
lean_dec(x_1);
return x_6;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__6___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = l_Nonos_Secure_step___elambda__6(x_1, x_2, x_3, x_4, x_5);
lean_dec(x_3);
lean_dec(x_2);
return x_6;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__7___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_Nonos_Secure_step___elambda__7(x_1, x_2, x_3, x_4);
lean_dec(x_2);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_step___elambda__9___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = l_Nonos_Secure_step___elambda__9(x_1, x_2, x_3, x_4);
lean_dec(x_2);
return x_5;
}
}
LEAN_EXPORT lean_object* l_Nonos_Secure_run(lean_object* x_1, lean_object* x_2) {
_start:
{
if (lean_obj_tag(x_2) == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; 
x_3 = lean_ctor_get(x_2, 0);
lean_inc(x_3);
x_4 = lean_ctor_get(x_2, 1);
lean_inc(x_4);
lean_dec(x_2);
x_5 = l_Nonos_Secure_step(x_1, x_3);
x_1 = x_5;
x_2 = x_4;
goto _start;
}
}
}
LEAN_EXPORT lean_object* l___private_Nonos_Secure_0__Nonos_Secure_step_match__1_splitter___rarg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7) {
_start:
{
switch (lean_obj_tag(x_1)) {
case 0:
{
lean_object* x_8; lean_object* x_9; lean_object* x_10; 
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_5);
lean_dec(x_4);
lean_dec(x_3);
x_8 = lean_ctor_get(x_1, 0);
lean_inc(x_8);
x_9 = lean_ctor_get(x_1, 1);
lean_inc(x_9);
lean_dec(x_1);
x_10 = lean_apply_2(x_2, x_8, x_9);
return x_10;
}
case 1:
{
lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; 
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_5);
lean_dec(x_4);
lean_dec(x_2);
x_11 = lean_ctor_get(x_1, 0);
lean_inc(x_11);
x_12 = lean_ctor_get(x_1, 1);
lean_inc(x_12);
x_13 = lean_ctor_get(x_1, 2);
lean_inc(x_13);
lean_dec(x_1);
x_14 = lean_apply_3(x_3, x_11, x_12, x_13);
return x_14;
}
case 2:
{
lean_object* x_15; lean_object* x_16; lean_object* x_17; 
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_5);
lean_dec(x_3);
lean_dec(x_2);
x_15 = lean_ctor_get(x_1, 0);
lean_inc(x_15);
x_16 = lean_ctor_get(x_1, 1);
lean_inc(x_16);
lean_dec(x_1);
x_17 = lean_apply_2(x_4, x_15, x_16);
return x_17;
}
case 3:
{
lean_object* x_18; lean_object* x_19; lean_object* x_20; 
lean_dec(x_7);
lean_dec(x_6);
lean_dec(x_4);
lean_dec(x_3);
lean_dec(x_2);
x_18 = lean_ctor_get(x_1, 0);
lean_inc(x_18);
x_19 = lean_ctor_get(x_1, 1);
lean_inc(x_19);
lean_dec(x_1);
x_20 = lean_apply_2(x_5, x_18, x_19);
return x_20;
}
case 4:
{
lean_object* x_21; lean_object* x_22; lean_object* x_23; 
lean_dec(x_7);
lean_dec(x_5);
lean_dec(x_4);
lean_dec(x_3);
lean_dec(x_2);
x_21 = lean_ctor_get(x_1, 0);
lean_inc(x_21);
x_22 = lean_ctor_get(x_1, 1);
lean_inc(x_22);
lean_dec(x_1);
x_23 = lean_apply_2(x_6, x_21, x_22);
return x_23;
}
default: 
{
lean_object* x_24; lean_object* x_25; 
lean_dec(x_6);
lean_dec(x_5);
lean_dec(x_4);
lean_dec(x_3);
lean_dec(x_2);
x_24 = lean_ctor_get(x_1, 0);
lean_inc(x_24);
lean_dec(x_1);
x_25 = lean_apply_1(x_7, x_24);
return x_25;
}
}
}
}
LEAN_EXPORT lean_object* l___private_Nonos_Secure_0__Nonos_Secure_step_match__1_splitter(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_alloc_closure((void*)(l___private_Nonos_Secure_0__Nonos_Secure_step_match__1_splitter___rarg), 7, 0);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Capability(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_Isolation(uint8_t builtin, lean_object*);
lean_object* initialize_Nonos_AntiRollback(uint8_t builtin, lean_object*);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_Nonos_Secure(uint8_t builtin, lean_object* w) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Capability(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_Isolation(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Nonos_AntiRollback(builtin, lean_io_mk_world());
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_Nonos_Secure_step___closed__1 = _init_l_Nonos_Secure_step___closed__1();
lean_mark_persistent(l_Nonos_Secure_step___closed__1);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
