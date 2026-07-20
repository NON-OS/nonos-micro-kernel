#ifndef _NONOS_MATH_H
#define _NONOS_MATH_H
#define NAN __builtin_nanf("")
#define INFINITY __builtin_inff()
#define HUGE_VAL __builtin_huge_val()
#define M_PI 3.14159265358979323846
#define isnan(x) __builtin_isnan(x)
#define isinf(x) __builtin_isinf(x)
#define isfinite(x) __builtin_isfinite(x)
#define signbit(x) __builtin_signbit(x)
#define isnormal(x) __builtin_isnormal(x)
double acos(double); double acosh(double); double asin(double); double asinh(double);
double atan(double); double atanh(double); double atan2(double,double); double cbrt(double);
double ceil(double); double cos(double); double cosh(double); double exp(double);
double expm1(double); double fabs(double); double floor(double); double fmod(double,double);
double frexp(double,int*); double hypot(double,double); double log(double); double log2(double);
double log10(double); double log1p(double); double modf(double,double*); double pow(double,double);
double round(double); double scalbn(double,int); double sin(double); double sinh(double);
double sqrt(double); double tan(double); double tanh(double); double trunc(double);
double ldexp(double,int); double nearbyint(double); double rint(double); double copysign(double,double);
float fabsf(float); float floorf(float);
long lrint(double); long long llrint(double); double nan(const char*);
#endif
