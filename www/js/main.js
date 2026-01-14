/**
 * WASM Pixel Canvas Demo - 主应用逻辑
 */

import {
    initWasm,
    createBuffer,
    renderBufferToCanvas,
    hexToPackedRGBA,
    FORMAT,
} from './wasm-utils.js';

// ============ 应用状态 ============
const state = {
    buffer: null,
    format: FORMAT.RGBA,
    startColor: '#ff0000',
    endColor: '#0000ff',
    canvasWidth: 400,
    canvasHeight: 300,
};

// ============ DOM 元素 ============
const elements = {
    canvas: null,
    formatButtons: {},
    startColorInput: null,
    endColorInput: null,
    runTestButton: null,
    infoDisplay: null,
};

// ============ 初始化 ============
async function initialize() {
    // 缓存 DOM 元素
    elements.canvas = document.getElementById('canvas');
    elements.formatButtons = {
        [FORMAT.RGBA]: document.getElementById('btnRgba'),
        [FORMAT.RGB]: document.getElementById('btnRgb'),
        [FORMAT.GRAYSCALE]: document.getElementById('btnGray'),
    };
    elements.startColorInput = document.getElementById('startColor');
    elements.endColorInput = document.getElementById('endColor');
    elements.runTestButton = document.getElementById('runTest');
    elements.infoDisplay = document.getElementById('info');

    // 初始化 WASM
    await initWasm();
    console.log('✅ WASM initialized');

    // 创建初始 buffer
    resetBuffer();

    // 绑定事件
    bindEvents();

    // 更新提示信息
    updateInfo();
}

// ============ Buffer 管理 ============
function resetBuffer() {
    // 释放旧 buffer（GC 会处理）
    state.buffer = createBuffer(state.canvasWidth, state.canvasHeight, state.format);

    // 清空 canvas
    const ctx = elements.canvas.getContext('2d');
    ctx.clearRect(0, 0, state.canvasWidth, state.canvasHeight);
}

// ============ 格式切换 ============
function setFormat(format) {
    state.format = format;

    // 更新按钮状态
    Object.entries(elements.formatButtons).forEach(([fmt, btn]) => {
        btn.classList.toggle('active', parseInt(fmt) === format);
    });

    // 重置 buffer
    resetBuffer();
    updateInfo();
}

// ============ 渐变测试 ============
function runGradientTest() {
    if (!state.buffer) return;

    // 获取颜色并转换为 packed RGBA
    const startPacked = hexToPackedRGBA(state.startColor);
    const endPacked = hexToPackedRGBA(state.endColor);

    // 调用 Rust 生成渐变
    state.buffer.test_gradient(startPacked, endPacked);

    // 渲染到 canvas
    renderBufferToCanvas(state.buffer, elements.canvas, state.format);

    console.log(`🎨 Rendered gradient: ${state.startColor} → ${state.endColor}`);
}

// ============ 事件绑定 ============
function bindEvents() {
    // 格式切换按钮
    elements.formatButtons[FORMAT.RGBA].addEventListener('click', () => setFormat(FORMAT.RGBA));
    elements.formatButtons[FORMAT.RGB].addEventListener('click', () => setFormat(FORMAT.RGB));
    elements.formatButtons[FORMAT.GRAYSCALE].addEventListener('click', () => setFormat(FORMAT.GRAYSCALE));

    // 运行测试按钮
    elements.runTestButton.addEventListener('click', runGradientTest);

    // 颜色选择器
    elements.startColorInput.addEventListener('input', (e) => {
        state.startColor = e.target.value;
    });
    elements.endColorInput.addEventListener('input', (e) => {
        state.endColor = e.target.value;
    });
}

// ============ 信息展示 ============
function updateInfo() {
    const formatName = {
        [FORMAT.RGBA]: 'RGBA (Zero Copy)',
        [FORMAT.RGB]: 'RGB',
        [FORMAT.GRAYSCALE]: 'Grayscale',
    };

    const bufferSize = state.canvasWidth * state.canvasHeight * state.format;
    const sizeKB = (bufferSize / 1024).toFixed(1);

    elements.infoDisplay.innerHTML = `
        Format: <span class="info-highlight">${formatName[state.format]}</span> | 
        Buffer: <span class="info-highlight">${sizeKB} KB</span> | 
        Size: <span class="info-highlight">${state.canvasWidth}×${state.canvasHeight}</span>
    `;
}

// ============ 启动应用 ============
initialize().catch(console.error);
