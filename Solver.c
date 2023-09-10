#include <stdint.h>
#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include "hashmap.h"

struct arrayRes {
  int *arr;
  ssize_t len;
  long res;
};

uint64_t hash_arr(const void* item, uint64_t seed0, uint64_t seed1) {
  const struct arrayRes* arr = item;
  return hashmap_sip(arr->arr, arr->len * sizeof(int), seed0, seed1);
}

int compare_arr(const void* a, const void* b, void* udata) {
  const struct arrayRes* aa = a;
  const struct arrayRes* bb = b;
  if (aa->len != bb->len) {
    return aa->len - bb-> len;
  }
  for (int i = aa->len; i > 0; --i) {
    if (aa->arr[i] != bb->arr[i]) {
      return aa->arr[i] - bb->arr[i];
    }
  }
  return 0;
}

long combs(int arr[], ssize_t len, const struct hashmap* mem) {
  if (len < 1) return 0;
  if (len == 1 && arr[0] == 1) {
    return 1;
  }
  struct arrayRes a = {
    .arr=arr,
    .len=len,
  };
  struct arrayRes* cache = hashmap_get(mem, &a);
  if (cache != NULL) {
    return cache->res;
  }
  long out = 0;
  for (int i = 0; i < len; i++) {
    int diff;
    if (i < len-1) {
      diff = arr[i] - arr[i+1];
    } else {
      diff = arr[i];
    }
    if (diff > 0) {
      arr[i] -= 1;
      int j;
      for (j=0; arr[j] != 0 && j < len; j++) {}
      long res = combs(arr, j, mem);
      out += diff * res; 
      arr[i] += 1;
    } 
  }
  a.res = out;
  hashmap_set(mem, &a);
  return out;  
}

int main(int argc, char *argv[]) {
  int arr[argc-2];
  for (int i = 1; i < argc; i++) {
    arr[i-1] = atoi(argv[i]);
    printf("%i ", arr[i-1]);
  }
  printf("\n");
  const struct hashmap* mem = hashmap_new(sizeof(struct arrayRes), 0, 0, 0, hash_arr, compare_arr, NULL, NULL);
  clock_t start = clock(), diff;
  long res = combs(arr, argc-1, mem);
  diff = clock() - start;
  int usec = diff * 1000000 / CLOCKS_PER_SEC;
  printf("Took: %i ms %i µs\n", usec/1000, usec%1000);
  printf("Result: %li\n", res);
}
