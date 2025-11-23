#include "area.h"

#define MAX_AREAS 128
static Area area_stack[MAX_AREAS];
static int area_depth = 0;

void area_start() {
    if (area_depth >= MAX_AREAS) {
        fprintf(stderr, "Too many nested areas!\n");
        exit(1);
    }

    area_stack[area_depth].allocations = NULL;
    area_depth++;
}

void area_end() {
    if (area_depth <= 0) {
        fprintf(stderr, "No area end!\n");
        exit(1);
    }

    area_depth--;

    AllocNode* node = area_stack[area_depth].allocations;
    while (node)
    {
        free(node->ptr);
        AllocNode* next = node->next;
        free(node);
        node = next;
    }
}

void* area_register_alloc(void* ptr) {
    if (area_depth <= 0) {
        fprintf(stderr, "No area end!\n");
        exit(1);
    }
    
    AllocNode* node = malloc(sizeof(AllocNode));
    node->ptr = ptr;
    node->next = area_stack[area_depth - 1].allocations;

    area_stack[area_depth - 1].allocations = node;
    return ptr;
}

void* area_alloc(size_t size) {
    void* block = malloc(size);
    return area_register_alloc(block);
}

void* area_memdup(const void* src, size_t size) {
    void* block = area_alloc(size);
    memcpy(block, src, size);
    return block;
}