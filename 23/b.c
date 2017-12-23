#include <stdint.h>
#include <stdio.h>
#include <math.h>

typedef int64_t i64;

int main() {
    const i64 start = 65 * 100 + 100000;
    const i64 end = start + 17000;
    i64 primes = 0;

    for (i64 i = start; i != end+17; i += 17) {
        int is_prime = 1;

        for (int j = 2; j < sqrt(i)+1; ++j) {
            if (i % j == 0) {
                is_prime = 0;
                break;
            }
        }

        if (is_prime == 0) {
            primes++;
        }
    }
    printf("%lu\n", primes);
}
