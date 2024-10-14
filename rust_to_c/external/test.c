#include "test.h"

double addValuesTogheter(SomeThingCool* someThingCool) {
    // Just a random change to the struct.
    someThingCool->t = 10;
    return (double)someThingCool->t + someThingCool->d ;
}
