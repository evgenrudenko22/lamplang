#include "math.h"
#define PI 3.14159265358979323846
#define E 2.71828182845904523536

double square(double x) {
    return x * x;
}
double pow(double base, double exp) {
    int c_exp = round(exp);
    if (c_exp == 0) return 1.0;

    double result = 1.0;
    int abs_exp = c_exp < 0 ? -c_exp : c_exp;

    for (int i = 0; i < abs_exp; i++) {
        result *= base;
    }

    return c_exp < 0 ? 1.0 / result : result;
}
double abs_d(double x) {
    return x < 0 ? -x : x;
}
double sqrt(double x) {
    if (x < 0) return -1.0;
    if (x == 0) return 0;
    
    double guess = x / 2.0;
    double epsilon = 0.00000001;

    while (abs_d(guess * guess - x) > epsilon)
    {
        guess = (guess + x / guess) / 2.0;
    }

    return guess;    
}
double ln(double x) {
    if (x <= 0) return -1.0;

    double y = (x - 1.0) / (x + 1.0);
    double y_sq = y * y;
    double result = 0.0;
    double term = y;

    for (int n = 0; n < 100; n++) {
        result += term / (2 * n + 1);
        term *= y_sq;
    }

    return 2.0 * result;
}
double log2(double x) {
    return ln(x) / ln(2.0);
}
double log10(double x) {
    return ln(x) / ln(10.0);
}
double sin(double x) {
    while (x > 2 * PI) x -= 2 * PI;
    while (x < -2 * PI) x += 2 * PI;

    double result = 0.0;
    double term = x;

    for (int n = 0; n < 15; n++) {
        result += term;
        term *= -x * x / ((2 * n + 2) * (2 * n + 2));
    }

    return result;
}
double cos(double x) {
    return sin(PI / 2.0 - x);
}
double tan(double x) {
    double cos_val = cos(x);
    if (abs_d(cos_val) < 0.00000001) {
        return 0.0;
    }
    return sin(x) / cos_val;
}
double round(double x) {
    if (x >= 0) {
        return (double)((int)(x + 0.5));
    } else {
        return (double)((int)(x - 0.5));
    }
}
double floor(double x) {
    int int_part = (int)x;
    if (x >= 0 || x == (double)int_part) {
        return (double) int_part;
    }
    return (double)(int_part - 1);
}
double ceil(double x) {
    int int_part = (int)x;
    if (x <= 0 || x == (double)int_part) {
        return (double)int_part;
    }
    return (double)(int_part + 1);
}