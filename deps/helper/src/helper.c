/*
 A quick and evil C file to convert a rust FFI va-args function into a call with va_list passed
 */
#include <stdarg.h>

extern void inbound(void *context, unsigned int count, va_list args);

void dispatch(void* context, unsigned int count, ...)
{
	va_list	args;
	va_start(args, count);
	inbound(context, count, args);
	va_end(args);
}
