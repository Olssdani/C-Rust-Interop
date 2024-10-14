/* THIS FILE IS GENERATED, DO NOT EDIT!*/

#pragma once

#include <cstdint>

struct TestStruct {
    float x;
    double y;
    int32_t z;
};

extern "C" {

bool start_web();

bool has_health();

int32_t add(int32_t a, int32_t b);

double add_togheter(const TestStruct *data);

int32_t get_env(const char *env_variable, char *output, int32_t size);

}  // extern "C"
