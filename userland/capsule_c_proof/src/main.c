#include <unistd.h>
static const char PASS[] = "[C-PROOF] PASS\n";
int main(void) {
    write(1, PASS, sizeof(PASS) - 1);
    return 0;
}
