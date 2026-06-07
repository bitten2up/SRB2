#define MAX_NUM_VECTORS			(64 * 1024)
#define VCACHE_NUM_BUFFERS		3

// Defined in r_nds3d.c. Without this declaration the call below was treated
// as returning the C89 implicit `int`, which LTO (cross-TU codegen) then
// mismatches against the real `void *` return — corrupts geometryBuf on
// 3DS and shows up as a crash/hang at first frame.
extern void *I_InitVertexBuffer(const size_t geoBufSize);

extern FOutVector *	geometryBuf;
extern size_t		geometryBufIndex;
extern size_t		geometryBufSlot;

extern void HWR_SwapVertexBuffer();

inline FOutVector *HWR_AllocVertexBuffer(size_t numVectors)
{
	const size_t remaining = MAX_NUM_VECTORS - (geometryBufIndex - geometryBufSlot * MAX_NUM_VECTORS);
	const size_t bufIndex = geometryBufIndex;
	FOutVector *vectors;

#ifdef DIAGNOSTIC
	if(remaining < numVectors)
	{
		I_Error("geobuf too small!\n");
		return NULL;
	}
#endif

	vectors = &geometryBuf[bufIndex];
	
	geometryBufIndex += numVectors;

	return vectors;
}
