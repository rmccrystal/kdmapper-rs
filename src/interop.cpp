#include "kdmapper.hpp"

extern "C" NTSTATUS map_driver(BYTE* data, bool free, bool mdlMode, bool passAllocationPtr, NTSTATUS* exitCode, ULONG64 param1, ULONG64 param2) {
    const auto handle = intel_driver::Load();
    if(!handle) {
        return -1;
    }

    const auto status = kdmapper::MapDriver(handle, data, param1, param2, free, true, mdlMode, passAllocationPtr, NULL, exitCode);

    intel_driver::Unload(handle);

    return status;
}