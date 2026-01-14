/**
 * WASM 工具模块
 * 提供 WASM 初始化、Buffer 渲染等功能
 */

import init, { SharedBuffer, ImageFormat } from '../../pkg/pixel_canvas.js';

/** @type {WebAssembly.Memory | null} */
let wasmMemory = null;

/**
 * 初始化 WASM 模块
 * @returns {Promise<{ memory: WebAssembly.Memory }>}
 */
export async function initWasm() {
    const wasm = await init();
    wasmMemory = wasm.memory;
    return wasm;
}

/**
 * 获取 WASM 内存
 * @returns {WebAssembly.Memory}
 */
export function getWasmMemory() {
    if (!wasmMemory) {
        throw new Error('WASM not initialized. Call initWasm() first.');
    }
    return wasmMemory;
}

/**
 * 图像格式枚举映射
 */
export const FORMAT = {
    GRAYSCALE: 1,
    RGB: 3,
    RGBA: 4,
};

/**
 * 创建 SharedBuffer 实例
 * @param {number} width
 * @param {number} height
 * @param {number} format - 1: Grayscale, 3: RGB, 4: RGBA
 * @returns {SharedBuffer}
 */
export function createBuffer(width, height, format = FORMAT.RGBA) {
    return new SharedBuffer(width, height, format);
}

/**
 * 将颜色从十六进制字符串转换为 RGBA packed u32
 * @param {string} hex - 例如 "#ff0000"
 * @param {number} alpha - 0-255
 * @returns {number} 0xRRGGBBAA
 */
export function hexToPackedRGBA(hex, alpha = 255) {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    return ((r << 24) | (g << 16) | (b << 8) | alpha) >>> 0;
}

/**
 * 将 SharedBuffer 渲染到 Canvas
 * @param {SharedBuffer} buffer
 * @param {HTMLCanvasElement} canvas
 * @param {number} format
 */
export function renderBufferToCanvas(buffer, canvas, format) {
    const ctx = canvas.getContext('2d');
    const width = buffer.width();
    const height = buffer.height();
    const ptr = buffer.ptr();
    const len = buffer.len();
    const memory = getWasmMemory().buffer;

    // 直接从 Rust buffer 读取数据
    const rustBuffer = new Uint8Array(memory, ptr, len);

    if (format === FORMAT.RGBA) {
        // RGBA - 零拷贝快速路径
        const clampedArray = new Uint8ClampedArray(memory, ptr, len);
        const imageData = new ImageData(clampedArray, width, height);
        ctx.putImageData(imageData, 0, 0);
    } else if (format === FORMAT.RGB) {
        // RGB - 需要扩展为 RGBA
        const imageData = ctx.createImageData(width, height);
        const data = imageData.data;
        
        for (let i = 0, j = 0; i < len; i += 3, j += 4) {
            data[j] = rustBuffer[i];       // R
            data[j + 1] = rustBuffer[i + 1]; // G
            data[j + 2] = rustBuffer[i + 2]; // B
            data[j + 3] = 255;             // A
        }
        ctx.putImageData(imageData, 0, 0);
    } else if (format === FORMAT.GRAYSCALE) {
        // Grayscale - 需要扩展为 RGBA
        const imageData = ctx.createImageData(width, height);
        const data = imageData.data;
        
        for (let i = 0, j = 0; i < len; i++, j += 4) {
            const val = rustBuffer[i];
            data[j] = val;     // R
            data[j + 1] = val; // G
            data[j + 2] = val; // B
            data[j + 3] = 255; // A
        }
        ctx.putImageData(imageData, 0, 0);
    }
}

// 重新导出 WASM 类型供外部使用
export { SharedBuffer, ImageFormat };
