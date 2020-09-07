#include <stdio.h>
#include <windows.h>
#include <excpt.h>
#include <iostream>
#include <cstdint>

extern "C" __declspec(dllexport) bool try_catch_memcpy(void* dst, const void* src, size_t len) {
	__try
	{
		std::memcpy(dst, src, len);
		return 1;
	}
	__except (GetExceptionCode() == EXCEPTION_IN_PAGE_ERROR ?
		EXCEPTION_EXECUTE_HANDLER : EXCEPTION_CONTINUE_SEARCH)
	{
		return 0;
	}
}