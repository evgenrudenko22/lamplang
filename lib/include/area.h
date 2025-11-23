#ifndef LAMP_AREA_H
#define LAMP_AREA_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct AllocNode {
    void* ptr;
    struct AllocNode* next;
} AllocNode;

typedef struct Area {
    AllocNode* allocations;
} Area;

void area_start(void);
void area_end(void);

void* area_register_alloc(void* ptr);
void* area_alloc(size_t size);
void* area_memdup(const void* src, size_t size);

#endif // !LAMP_AREA_H