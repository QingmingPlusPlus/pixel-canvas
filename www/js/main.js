/**
 * WASM Pixel Canvas Demo - 主应用逻辑
 */

import {
    initWasm,
    createBuffer,
    FORMAT,
} from './wasm-utils.js';

// ============ 应用状态 ============
const state = {
    buffer: null,
    format: FORMAT.RGBA,
    canvasWidth: 400,
    canvasHeight: 300,
};

// ============ DOM 元素 ============
const elements = {
    canvas: null,
    formatButtons: {},
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



// ============ 事件绑定 ============
function bindEvents() {
    // 格式切换按钮
    elements.formatButtons[FORMAT.RGBA].addEventListener('click', () => setFormat(FORMAT.RGBA));
    elements.formatButtons[FORMAT.RGB].addEventListener('click', () => setFormat(FORMAT.RGB));
    elements.formatButtons[FORMAT.GRAYSCALE].addEventListener('click', () => setFormat(FORMAT.GRAYSCALE));


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
