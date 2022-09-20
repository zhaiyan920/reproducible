

struct TestData {
  int data;
} __attribute__((aligned(16)));

struct TestData data[1];

int prog(void *ctx) {
  data[0].data = 2;
  return 0;
}
