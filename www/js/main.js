/**
 * WASM Sprite Transform Demo - 主应用逻辑
 */

import {
    initWasm,
    getWasmMemory,
} from './wasm-utils.js';

import init, { WasmScene } from '../../pkg/pixel_canvas.js';

// ============ 应用状态 ============
const state = {
    scene: null,
    spriteIndex: null,
    canvasWidth: 400,
    canvasHeight: 300,
};

// ============ DOM 元素 ============
const elements = {
    canvas: null,
    infoDisplay: null,
    buttons: {},
};

// ============ 初始化 ============
async function initialize() {
    // 缓存 DOM 元素
    elements.canvas = document.getElementById('canvas');
    elements.infoDisplay = document.getElementById('info');

    // 缓存按钮
    elements.buttons = {
        left: document.getElementById('btnLeft'),
        right: document.getElementById('btnRight'),
        up: document.getElementById('btnUp'),
        down: document.getElementById('btnDown'),
        rotateLeft: document.getElementById('btnRotateLeft'),
        rotateRight: document.getElementById('btnRotateRight'),
        scaleUp: document.getElementById('btnScaleUp'),
        scaleDown: document.getElementById('btnScaleDown'),
        scaleX: document.getElementById('btnScaleX'),
        scaleY: document.getElementById('btnScaleY'),
        reset: document.getElementById('btnReset'),
        anchorCenter: document.getElementById('btnAnchorCenter'),
        anchorCorner: document.getElementById('btnAnchorCorner'),
    };

    // 初始化 WASM
    await initWasm();
    console.log('✅ WASM initialized');

    // 创建场景
    createScene();

    // 绑定事件
    bindEvents();

    // 初始渲染
    render();
}

// ============ 场景创建 ============
function createScene() {
    // 创建场景
    state.scene = new WasmScene(state.canvasWidth, state.canvasHeight);

    // 设置背景颜色 (深蓝色)
    state.scene.set_background_color(26, 26, 46, 255);

    // 添加一个长方形精灵 (120x80, 紫色)
    state.spriteIndex = state.scene.add_rectangle(120, 80, 124, 58, 237, 255);

    // 设置初始位置（居中）
    state.scene.set_sprite_position(
        state.spriteIndex,
        state.canvasWidth / 2,
        state.canvasHeight / 2
    );

    console.log('✅ Scene created with rectangle sprite');
}

// ============ 渲染 ============
function render() {
    // 渲染场景
    state.scene.render();

    // 获取场景数据并绘制到 canvas
    const ptr = state.scene.ptr();
    const len = state.scene.len();
    const memory = getWasmMemory().buffer;

    const ctx = elements.canvas.getContext('2d');
    const clampedArray = new Uint8ClampedArray(memory, ptr, len);
    const imageData = new ImageData(clampedArray, state.canvasWidth, state.canvasHeight);
    ctx.putImageData(imageData, 0, 0);

    // 更新信息
    updateInfo();
}

// ============ 事件绑定 ============
function bindEvents() {
    const step = 20; // 平移步长
    const scaleFactor = 1.2; // 缩放因子
    const rotateDegrees = 15; // 旋转角度

    // 平移
    elements.buttons.left.addEventListener('click', () => {
        state.scene.translate_sprite(state.spriteIndex, -step, 0);
        render();
    });

    elements.buttons.right.addEventListener('click', () => {
        state.scene.translate_sprite(state.spriteIndex, step, 0);
        render();
    });

    elements.buttons.up.addEventListener('click', () => {
        state.scene.translate_sprite(state.spriteIndex, 0, -step);
        render();
    });

    elements.buttons.down.addEventListener('click', () => {
        state.scene.translate_sprite(state.spriteIndex, 0, step);
        render();
    });

    // 旋转
    elements.buttons.rotateLeft.addEventListener('click', () => {
        state.scene.rotate_sprite(state.spriteIndex, -rotateDegrees);
        render();
    });

    elements.buttons.rotateRight.addEventListener('click', () => {
        state.scene.rotate_sprite(state.spriteIndex, rotateDegrees);
        render();
    });

    // 缩放
    elements.buttons.scaleUp.addEventListener('click', () => {
        state.scene.scale_sprite_by(state.spriteIndex, scaleFactor, scaleFactor);
        render();
    });

    elements.buttons.scaleDown.addEventListener('click', () => {
        state.scene.scale_sprite_by(state.spriteIndex, 1 / scaleFactor, 1 / scaleFactor);
        render();
    });

    elements.buttons.scaleX.addEventListener('click', () => {
        state.scene.scale_sprite_by(state.spriteIndex, scaleFactor, 1);
        render();
    });

    elements.buttons.scaleY.addEventListener('click', () => {
        state.scene.scale_sprite_by(state.spriteIndex, 1, scaleFactor);
        render();
    });

    // 重置
    elements.buttons.reset.addEventListener('click', () => {
        state.scene.reset_sprite_transform(state.spriteIndex);
        // 重新设置初始位置（居中）
        state.scene.set_sprite_position(
            state.spriteIndex,
            state.canvasWidth / 2,
            state.canvasHeight / 2
        );
        // 重置为中心锚点
        state.scene.set_sprite_anchor(state.spriteIndex, 0.5, 0.5);
        render();
    });

    // 锚点设置
    elements.buttons.anchorCenter.addEventListener('click', () => {
        state.scene.set_sprite_anchor(state.spriteIndex, 0.5, 0.5);
        render();
    });

    elements.buttons.anchorCorner.addEventListener('click', () => {
        state.scene.set_sprite_anchor(state.spriteIndex, 0, 0);
        render();
    });
}

// ============ 信息展示 ============
function updateInfo() {
    const x = state.scene.get_sprite_position_x(state.spriteIndex).toFixed(1);
    const y = state.scene.get_sprite_position_y(state.spriteIndex).toFixed(1);
    const rotation = (state.scene.get_sprite_rotation(state.spriteIndex) * 180 / Math.PI).toFixed(1);
    const scaleX = state.scene.get_sprite_scale_x(state.spriteIndex).toFixed(2);
    const scaleY = state.scene.get_sprite_scale_y(state.spriteIndex).toFixed(2);

    elements.infoDisplay.innerHTML = `
        位置: <span class="info-highlight">(${x}, ${y})</span> | 
        旋转: <span class="info-highlight">${rotation}°</span> | 
        缩放: <span class="info-highlight">(${scaleX}, ${scaleY})</span>
    `;
}

// ============ 启动应用 ============
initialize().catch(console.error);
