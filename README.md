a simple proof of concept of rendering digital waveforms on the gpu.

![example image](/images/img.png)

this image is rendered directly from an array of 4 u32s, all done on the gpu, using only a vertex and a fragment shader, in a single draw call. a full application would need a lot more functionality obviously but hopefully the idea is clear here.
