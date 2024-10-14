#include <stdio.h>
#include <string>
#include <chrono>
#include <thread>
#include "../include/bindings.h"

int main() {

    // Example 1 - Add two integers togheter
    {
        int32_t res = add(2, 5);
        printf("Example 1: 2 + 5 = %i \n", res);
    }

    // Example 2 - Add a structs values togheter
    {
        TestStruct testStruct;
        testStruct.x = 1.0f;
        testStruct.y = 2.0;
        testStruct.z = 5;

        double res = add_togheter(&testStruct);
        printf("Added struct togheter: %f \n", res);
    }

    // Example 3 - Get the value store in the env variable 'LIBCLANG_PATH'
    {

        int32_t size = get_env("LIBCLANG_PATH", nullptr, 0);
        printf("Size of env variable: %i \n", size);
        std::string env;
        env.resize(size);

        printf("Size of env variable: %i \n", env.size());
        size = get_env("LIBCLANG_PATH", env.data(), static_cast<int32_t>(env.size()));
        printf("Env variable value: %s \n", env.c_str());
    }


    // Example 4 - Start up a small Rest server, wait for one message than exit.
    start_web();
    while(!has_health()) {
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }

    // Should have a stop function but not implemented.
    std::printf("Got health message");

    return 0; 
}