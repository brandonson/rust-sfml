/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![allow(non_snake_case, non_camel_case_types)]

extern crate libc;
extern crate sfml_types;
extern crate csfml_system_sys;
extern crate csfml_window_sys;

use libc::{c_char, c_float, c_int, c_uint, c_void, c_uchar, size_t};
use sfml_types::*;
use csfml_window_sys::*;
use csfml_system_sys::*;

pub use sfPrimitiveType::*;

/// Glyph describes a glyph (a visual character)
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct Glyph {
    /// Offset to move horizontically to the next character.
    pub advance: c_float,
    /// Bounding rectangle of the glyph, in coordinates relative to the baseline.
    pub bounds: IntRect,
    /// Texture coordinates of the glyph inside the font's texture.
    pub texture_rect: IntRect
}

/**
 * Types of primitives that a VertexArray can render
 *
 * Points and lines have no area, therefore their thickness
 * will always be 1 pixel, regardless the current transform
 * and view.
 *
 */
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum sfPrimitiveType {
    /// List of individual points.
    sfPoints,
    /// List of individual lines.
    sfLines,
    /// List of connected lines, a point uses the previous point to form a line.
    sfLinesStrip,
    /// List of individual triangles.
    sfTriangles,
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    sfTrianglesStrip,
    /// List of connected triangles, a point uses the common center and the previous point to form a triangle.
    sfTrianglesFan,
    /// List of individual quads.
    sfQuads
}

/**
 * BlendMode factor types
 */
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum sfBlendFactor {
    sfBlendFactorZero,
    sfBlendFactorOne,
    sfBlendFactorSrcColor,
    sfBlendFactorOneMinusSrcColor,
    sfBlendFactorDstColor,
    sfBlendFactorOneMinusDstColor,
    sfBlendFactorSrcAlpha,
    sfBlendFactorOneMinusSrcAlpha,
    sfBlendFactorDstAlpha,
    sfBlendFactorOneMinusDstAlpha,
}

/**
 * BlendMode equations
 */
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum sfBlendEquation {
    sfBlendEquationAdd,
    sfBlendEquationSubtract
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct sfBlendMode {
    pub colorSrcFactor: sfBlendFactor,
    pub colorDstFactor: sfBlendFactor,
    pub colorEquation: sfBlendEquation,
    pub alphaSrcFactor: sfBlendFactor,
    pub alphaDstFactor: sfBlendFactor,
    pub alphaEquation: sfBlendEquation,
}

#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub struct sfTransform {
    pub a00: f32,
    pub a01: f32,
    pub a02: f32,
    pub a10: f32,
    pub a11: f32,
    pub a12: f32,
    pub a20: f32,
    pub a21: f32,
    pub a22: f32
}

#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct sfVertex {
    /// 2D position of the vertex
    pub position: Vector2f,
    /// Color of the vertex.
    pub color: sfColor,
    /// Coordinates of the texture's pixel to map to the vertex.
    pub tex_coords: Vector2f
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct sfColor {
    /// The red composant of the color
    pub red: u8,
    /// The green composant of the color
    pub green: u8,
    /// The blue composant of the color
    pub blue: u8,
    /// The alpha composant of the color
    pub alpha: u8
}

pub enum sfRenderWindow {}

extern "C" {
    pub fn sfRenderWindow_create(mode: sfVideoMode, title: *const c_char, style: c_uint, settings: *const sfContextSettings) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_createUnicode(mode: sfVideoMode, title: *const u32, style: c_uint, settings: *const sfContextSettings) -> *mut sfRenderWindow;
    //fn sfRenderWindow_createFromHandle(handle: sfWindowHandle, settings: *mut sfContextSettings) -> *mut sfRenderWindow;
    pub fn sfRenderWindow_destroy(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_close(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_isOpen(renderWindow: *mut sfRenderWindow) -> sfBool;
    pub fn sfRenderWindow_getSettings(renderWindow: *mut sfRenderWindow) -> sfContextSettings;
    pub fn sfRenderWindow_pollEvent(renderWindow: *mut sfRenderWindow, event: *mut ::csfml_window_sys::sfEvent) -> sfBool;
    pub fn sfRenderWindow_waitEvent(renderWindow: *mut sfRenderWindow, event: *mut ::csfml_window_sys::sfEvent) -> sfBool;
    pub fn sfRenderWindow_getPosition(renderWindow: *mut sfRenderWindow) -> Vector2i;
    pub fn sfRenderWindow_setPosition(renderWindow: *mut sfRenderWindow, position: Vector2i);
    pub fn sfRenderWindow_getSize(renderWindow: *mut sfRenderWindow) -> Vector2u;
    pub fn sfRenderWindow_setSize(renderWindow: *mut sfRenderWindow, size: Vector2u);
    pub fn sfRenderWindow_setTitle(renderWindow: *mut sfRenderWindow, title: *const c_char);
    pub fn sfRenderWindow_setUnicodeTitle(renderWindow: *mut sfRenderWindow, title: *const u32);
    pub fn sfRenderWindow_setIcon(renderWindow: *mut sfRenderWindow, width: c_uint, height: c_uint, pixels: *const u8);
    pub fn sfRenderWindow_setVisible(renderWindow: *mut sfRenderWindow, visible: sfBool);
    pub fn sfRenderWindow_setMouseCursorVisible(renderWindow: *mut sfRenderWindow, show: sfBool);
    pub fn sfRenderWindow_setVerticalSyncEnabled(renderWindow: *mut sfRenderWindow, enabled: sfBool);
    pub fn sfRenderWindow_setKeyRepeatEnabled(renderWindow: *mut sfRenderWindow, enabled: sfBool);
    pub fn sfRenderWindow_setActive(renderWindow: *mut sfRenderWindow, active: sfBool) -> sfBool;
    pub fn sfRenderWindow_display(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_setFramerateLimit(renderWindow: *mut sfRenderWindow, limit: c_uint);
    pub fn sfRenderWindow_setJoystickThreshold(renderWindow: *mut sfRenderWindow, treshold: c_float);
    // fn sfRenderWindow_getSystemHandle(renderWindow: *mut sfRenderWindow) -> sfWindowHandle;
    pub fn sfRenderWindow_clear(renderWindow: *mut sfRenderWindow, color: sfColor);
    pub fn sfRenderWindow_setView(renderWindow: *mut sfRenderWindow, view: *mut sfView);
    pub fn sfRenderWindow_getView(renderWindow: *mut sfRenderWindow) -> *mut sfView;
    pub fn sfRenderWindow_getDefaultView(renderWindow: *mut sfRenderWindow) -> *mut sfView;
    pub fn sfRenderWindow_getViewport(renderWindow: *mut sfRenderWindow, view: *mut sfView) -> IntRect;
    pub fn sfRenderWindow_mapPixelToCoords(renderWindow: *mut sfRenderWindow, point: Vector2i, view: *mut sfView) -> Vector2f;
    pub fn sfRenderWindow_mapCoordsToPixel(renderWindow: *mut sfRenderWindow, point: Vector2f, view: *mut sfView) -> Vector2i;
    pub fn sfRenderWindow_drawSprite(renderWindow: *mut sfRenderWindow, object: *mut sfSprite, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawText(renderWindow: *mut sfRenderWindow, object: *mut sfText, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawShape(renderWindow: *mut sfRenderWindow, object: *mut sfShape, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawCircleShape(renderWindow: *mut sfRenderWindow, object: *mut sfCircleShape, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawConvexShape(renderWindow: *mut sfRenderWindow, object: *mut sfConvexShape, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawRectangleShape(renderWindow: *mut sfRenderWindow, object: *mut sfRectangleShape, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawVertexArray(renderWindow: *mut sfRenderWindow, object: *mut sfVertexArray, states: *mut sfRenderStates);
    pub fn sfRenderWindow_drawPrimitives(renderWindow: *mut sfRenderWindow, vertices: *const sfVertex, vertexCount: c_uint, ttype: sfPrimitiveType, states: *mut sfRenderStates);
    pub fn sfRenderWindow_pushGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_popGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_resetGLStates(renderWindow: *mut sfRenderWindow);
    pub fn sfRenderWindow_capture(renderWindow: *mut sfRenderWindow) -> *mut sfImage;
    pub fn sfMouse_getPositionRenderWindow(relativeTo: *mut sfRenderWindow) -> Vector2i;
    pub fn sfMouse_setPositionRenderWindow(position: Vector2i, relativeTo: *mut sfRenderWindow);
}

pub enum sfCircleShape {}

extern "C" {
    pub fn sfCircleShape_create() -> *mut sfCircleShape;
    pub fn sfCircleShape_copy(shape: *mut sfCircleShape) -> *mut sfCircleShape;
    pub fn sfCircleShape_destroy(shape: *mut sfCircleShape);
    pub fn sfCircleShape_setPosition(shape: *mut sfCircleShape, position: Vector2f);
    pub fn sfCircleShape_setRotation(shape: *mut sfCircleShape, angle: c_float);
    pub fn sfCircleShape_setScale(shape: *mut sfCircleShape, scale: Vector2f);
    pub fn sfCircleShape_setOrigin(shape: *mut sfCircleShape, origin: Vector2f);
    pub fn sfCircleShape_getPosition(shape: *mut sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_getRotation(shape: *mut sfCircleShape) -> c_float;
    pub fn sfCircleShape_getScale(shape: *mut sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_getOrigin(shape: *mut sfCircleShape) -> Vector2f;
    pub fn sfCircleShape_move(shape: *mut sfCircleShape, offset: Vector2f);
    pub fn sfCircleShape_rotate(shape: *mut sfCircleShape, angle: c_float);
    pub fn sfCircleShape_scale(shape: *mut sfCircleShape, factors: Vector2f);
    pub fn sfCircleShape_getTransform(shape: *mut sfCircleShape) -> sfTransform;
    pub fn sfCircleShape_getInverseTransform(shape: *mut sfCircleShape) -> sfTransform;
    pub fn sfCircleShape_setTexture(shape: *mut sfCircleShape, texture: *mut sfTexture, reset_rect: sfBool);
    pub fn sfCircleShape_setTextureRect(shape: *mut sfCircleShape, rect: IntRect);
    pub fn sfCircleShape_setFillColor(shape: *mut sfCircleShape, color: sfColor);
    pub fn sfCircleShape_setOutlineColor(shape: *mut sfCircleShape, color: sfColor);
    pub fn sfCircleShape_setOutlineThickness(shape: *mut sfCircleShape, thickness: c_float);
    pub fn sfCircleShape_getTexture(shape: *mut sfCircleShape) -> *mut sfTexture;
    pub fn sfCircleShape_getTextureRect(shape: *mut sfCircleShape) -> IntRect;
    pub fn sfCircleShape_getFillColor(shape: *mut sfCircleShape) -> sfColor;
    pub fn sfCircleShape_getOutlineColor(shape: *mut sfCircleShape) -> sfColor;
    pub fn sfCircleShape_getOutlineThickness(shape: *mut sfCircleShape) -> c_float;
    pub fn sfCircleShape_getPointCount(shape: *mut sfCircleShape) -> c_uint;
    pub fn sfCircleShape_getPoint(shape: *mut sfCircleShape, index: c_uint) -> Vector2f;
    pub fn sfCircleShape_setRadius(shape: *mut sfCircleShape, radius: c_float);
    pub fn sfCircleShape_getRadius(shape: *mut sfCircleShape) -> c_float;
    pub fn sfCircleShape_setPointCount(shape: *mut sfCircleShape, count: c_uint);
    pub fn sfCircleShape_getLocalBounds(shape: *mut sfCircleShape) -> FloatRect;
    pub fn sfCircleShape_getGlobalBounds(shape: *mut sfCircleShape) -> FloatRect;
}

extern "C" {
    pub fn sfColor_fromRGB(red: u8, green: u8, blue: u8) -> sfColor;
    pub fn sfColor_fromRGBA(red: u8, green: u8, blue: u8, alpha: u8) -> sfColor;
    pub fn sfColor_add(color1: sfColor, color2: sfColor) -> sfColor;
    pub fn sfColor_modulate(color1: sfColor, color2: sfColor) -> sfColor;
}

pub enum sfConvexShape {}

extern "C" {
    pub fn sfConvexShape_create() -> *mut sfConvexShape;
    pub fn sfConvexShape_copy(shape: *mut sfConvexShape) -> *mut sfConvexShape;
    pub fn sfConvexShape_destroy(shape: *mut sfConvexShape);
    pub fn sfConvexShape_setPosition(shape: *mut sfConvexShape, position: Vector2f);
    pub fn sfConvexShape_setRotation(shape: *mut sfConvexShape, angle: c_float);
    pub fn sfConvexShape_setScale(shape: *mut sfConvexShape, scale: Vector2f);
    pub fn sfConvexShape_setOrigin(shape: *mut sfConvexShape, origin: Vector2f);
    pub fn sfConvexShape_getPosition(shape: *mut sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_getRotation(shape: *mut sfConvexShape) -> c_float;
    pub fn sfConvexShape_getScale(shape: *mut sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_getOrigin(shape: *mut sfConvexShape) -> Vector2f;
    pub fn sfConvexShape_move(shape: *mut sfConvexShape, offset: Vector2f);
    pub fn sfConvexShape_rotate(shape: *mut sfConvexShape, angle: c_float);
    pub fn sfConvexShape_scale(shape: *mut sfConvexShape, factors: Vector2f);
    pub fn sfConvexShape_getTransform(shape: *mut sfConvexShape) -> sfTransform;
    pub fn sfConvexShape_getInverseTransform(shape: *mut sfConvexShape) -> sfTransform;
    pub fn sfConvexShape_setTexture(shape: *mut sfConvexShape, texture: *mut sfTexture, reset_rect: sfBool);
    pub fn sfConvexShape_setTextureRect(shape: *mut sfConvexShape, rect: IntRect);
    pub fn sfConvexShape_setFillColor(shape: *mut sfConvexShape, color: sfColor);
    pub fn sfConvexShape_setOutlineColor(shape: *mut sfConvexShape, color: sfColor);
    pub fn sfConvexShape_setOutlineThickness(shape: *mut sfConvexShape, thickness: c_float);
    pub fn sfConvexShape_getTexture(shape: *mut sfConvexShape) -> *mut sfTexture;
    pub fn sfConvexShape_getTextureRect(shape: *mut sfConvexShape) -> IntRect;
    pub fn sfConvexShape_getFillColor(shape: *mut sfConvexShape) -> sfColor;
    pub fn sfConvexShape_getOutlineColor(shape: *mut sfConvexShape) -> sfColor;
    pub fn sfConvexShape_getOutlineThickness(shape: *mut sfConvexShape) -> c_float;
    pub fn sfConvexShape_getPointCount(shape: *mut sfConvexShape) -> c_uint;
    pub fn sfConvexShape_getPoint(shape: *mut sfConvexShape, index: c_uint) -> Vector2f;
    pub fn sfConvexShape_setPointCount(shape: *mut sfConvexShape, count: c_uint);
    pub fn sfConvexShape_setPoint(shape: *mut sfConvexShape, index: c_uint, point: Vector2f);
    pub fn sfConvexShape_getLocalBounds(shape: *mut sfConvexShape) -> FloatRect;
    pub fn sfConvexShape_getGlobalBounds(shape: *mut sfConvexShape) -> FloatRect;
}

pub enum sfFont {}

extern "C" {
    pub fn sfFont_createFromFile(filename: *const c_char) -> *mut sfFont;
    pub fn sfFont_copy(font: *mut sfFont) -> *mut sfFont;
    pub fn sfFont_createFromMemory(data: *const c_uchar, sizeInBytes: size_t) -> *mut sfFont;
    pub fn sfFont_createFromStream(stream: *mut sfInputStream) -> *mut sfFont;
    pub fn sfFont_destroy(font: *mut sfFont);
    pub fn sfFont_getGlyph(font: *mut sfFont, codepoint: u32, characterSize: c_uint, bold :sfBool) -> Glyph;
    pub fn sfFont_getKerning(font: *mut sfFont, first: u32, second: u32, characterSize: c_uint) -> c_int;
    pub fn sfFont_getLineSpacing(font: *mut sfFont, characterSize: c_uint) -> c_int;
    pub fn sfFont_getTexture(font: *mut sfFont, characterSize: c_uint) -> *mut sfTexture;
}

pub enum sfImage {}

extern "C" {
    pub fn sfImage_create(width: c_uint, height: c_uint) -> *mut sfImage;
    pub fn sfImage_createFromColor(width: c_uint, height: c_uint, color: sfColor) -> *mut sfImage;
    pub fn sfImage_createFromPixels(width: c_uint, height: c_uint, pixels: *const u8) -> *mut sfImage;
    pub fn sfImage_createFromFile(filename: *const c_char) -> *mut sfImage;
    pub fn sfImage_createFromMemory(data: *const c_uchar, size: size_t) -> *mut sfImage;
    pub fn sfImage_createFromStream(stream: *mut sfInputStream) -> *mut sfImage;
    pub fn sfImage_copy(image: *mut sfImage) -> *mut sfImage;
    pub fn sfImage_destroy(image: *mut sfImage);
    pub fn sfImage_saveToFile(image: *mut sfImage, filename: *const c_char) -> sfBool;
    pub fn sfImage_getSize(image: *mut sfImage) -> Vector2u;
    pub fn sfImage_createMaskFromColor(image: *mut sfImage, color: sfColor, alpha: u8);
    pub fn sfImage_copyImage(image: *mut sfImage, source: *mut sfImage, destX: c_uint, destY: c_uint, sourceRect: IntRect, applyAlpha: sfBool);
    pub fn sfImage_setPixel(image: *mut sfImage, x: c_uint, y: c_uint, color: sfColor);
    pub fn sfImage_getPixel(image: *mut sfImage, x: c_uint, y: c_uint) -> sfColor;
    pub fn sfImage_getPixelsPtr(image: *mut sfImage) -> *mut u8;
    pub fn sfImage_flipHorizontally(image: *mut sfImage);
    pub fn sfImage_flipVertically(image: *mut sfImage);
}

extern "C" {
    pub fn IntRect_contains(rect: *const IntRect, x: c_int, y: c_int) -> sfBool;
    pub fn IntRect_intersects(rect1: *const IntRect, rect2: *const IntRect, intersectons: *const IntRect) -> sfBool;
    pub fn FloatRect_intersects(rect1: *const FloatRect, rect2: *const FloatRect, intersectons: *const FloatRect) -> sfBool;
    pub fn FloatRect_contains(rect: *const FloatRect, x: f32, y: f32) -> sfBool;
}

pub enum sfRectangleShape {}

extern "C" {
    pub fn sfRectangleShape_create() -> *mut sfRectangleShape;
    pub fn sfRectangleShape_copy(shape: *mut sfRectangleShape) -> *mut sfRectangleShape;
    pub fn sfRectangleShape_destroy(shape: *mut sfRectangleShape);
    pub fn sfRectangleShape_setPosition(shape: *mut sfRectangleShape, position: Vector2f);
    pub fn sfRectangleShape_setRotation(shape: *mut sfRectangleShape, angle: c_float);
    pub fn sfRectangleShape_setScale(shape: *mut sfRectangleShape, scale: Vector2f);
    pub fn sfRectangleShape_setOrigin(shape: *mut sfRectangleShape, origin: Vector2f);
    pub fn sfRectangleShape_getPosition(shape: *mut sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getRotation(shape: *mut sfRectangleShape) -> c_float;
    pub fn sfRectangleShape_getScale(shape: *mut sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getOrigin(shape: *mut sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_move(shape: *mut sfRectangleShape, offset: Vector2f);
    pub fn sfRectangleShape_rotate(shape: *mut sfRectangleShape, angle: c_float);
    pub fn sfRectangleShape_scale(shape: *mut sfRectangleShape, factors: Vector2f);
    pub fn sfRectangleShape_getTransform(shape: *mut sfRectangleShape) -> sfTransform;
    pub fn sfRectangleShape_getInverseTransform(shape: *mut sfRectangleShape) -> sfTransform;
    pub fn sfRectangleShape_setTexture(shape: *mut sfRectangleShape, texture: *mut sfTexture, reset_rect: sfBool);
    pub fn sfRectangleShape_setTextureRect(shape: *mut sfRectangleShape, rect: IntRect);
    pub fn sfRectangleShape_setFillColor(shape: *mut sfRectangleShape, color: sfColor);
    pub fn sfRectangleShape_setOutlineColor(shape: *mut sfRectangleShape, color: sfColor);
    pub fn sfRectangleShape_setOutlineThickness(shape: *mut sfRectangleShape, thickness: c_float);
    pub fn sfRectangleShape_getTexture(shape: *mut sfRectangleShape) -> *mut sfTexture;
    pub fn sfRectangleShape_getTextureRect(shape: *mut sfRectangleShape) -> IntRect;
    pub fn sfRectangleShape_getFillColor(shape: *mut sfRectangleShape) -> sfColor;
    pub fn sfRectangleShape_getOutlineColor(shape: *mut sfRectangleShape) -> sfColor;
    pub fn sfRectangleShape_getOutlineThickness(shape: *mut sfRectangleShape) -> c_float;
    pub fn sfRectangleShape_getPointCount(shape: *mut sfRectangleShape) -> c_uint;
    pub fn sfRectangleShape_getPoint(shape: *mut sfRectangleShape, index: c_uint) -> Vector2f;
    pub fn sfRectangleShape_setSize(shape: *mut sfRectangleShape, size: Vector2f);
    pub fn sfRectangleShape_getSize(shape: *mut sfRectangleShape) -> Vector2f;
    pub fn sfRectangleShape_getLocalBounds(shape: *mut sfRectangleShape) -> FloatRect;
    pub fn sfRectangleShape_getGlobalBounds(shape: *mut sfRectangleShape) -> FloatRect;
}

#[repr(C)]
pub struct sfRenderStates {
    pub blendMode: sfBlendMode,
    pub transform: sfTransform,
    pub texture: *mut sfTexture,
    pub shader: *mut sfShader
}

pub enum sfShader {}

extern "C" {
    pub fn sfShader_createFromFile(vertexShaderFilename: *const c_char, fragmentShaderFilename: *const c_char) -> *mut sfShader;
    pub fn sfShader_createFromMemory(vertexShader: *const c_char, fragmentShader: *const c_char) -> *mut sfShader;
    pub fn sfShader_createFromStream(vertexShaderStream: *mut sfInputStream, fragmentShaderStream: *mut sfInputStream) -> *mut sfShader;
    pub fn sfShader_destroy(shader: *mut sfShader)-> ();
    pub fn sfShader_setFloatParameter(shader: *mut sfShader, name: *const c_char, x: c_float);
    pub fn sfShader_setFloat2Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float);
    pub fn sfShader_setFloat3Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float, z: c_float);
    pub fn sfShader_setFloat4Parameter(shader: *mut sfShader, name: *const c_char, x: c_float, y: c_float, z: c_float, w: c_float);
    pub fn sfShader_setVector2Parameter(shader: *mut sfShader, name: *const c_char, vector: Vector2f);
    pub fn sfShader_setVector3Parameter(shader: *mut sfShader, name: *const c_char, vector: Vector3f);
    pub fn sfShader_setColorParameter(shader: *mut sfShader, name: *const c_char, color: sfColor);
    pub fn sfShader_setTransformParameter(shader: *mut sfShader, name: *const c_char, transform: sfTransform);
    pub fn sfShader_setTextureParameter(shader: *mut sfShader, name: *const c_char, texture: *mut sfTexture);
    pub fn sfShader_setCurrentTextureParameter(shader: *mut sfShader, name: *const c_char);
    pub fn sfShader_bind(shader: *mut sfShader);
    pub fn sfShader_isAvailable() -> sfBool;
}

pub enum sfRenderTexture {}

extern "C" {
    pub fn sfRenderTexture_create(width: c_uint, height: c_uint, depthBuffer: sfBool) -> *mut sfRenderTexture;
    pub fn sfRenderTexture_destroy(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_getSize(renderTexture: *mut sfRenderTexture) -> Vector2u;
    pub fn sfRenderTexture_setActive(renderTexture: *mut sfRenderTexture, active: sfBool) -> sfBool;
    pub fn sfRenderTexture_display(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_clear(renderTexture: *mut sfRenderTexture, color: sfColor);
    pub fn sfRenderTexture_setView(renderTexture: *mut sfRenderTexture, view: *mut sfView);
    pub fn sfRenderTexture_getView(renderTexture: *mut sfRenderTexture) -> *mut sfView;
    pub fn sfRenderTexture_getDefaultView(renderTexture: *mut sfRenderTexture) -> *mut sfView;
    pub fn sfRenderTexture_getViewport(renderTexture: *mut sfRenderTexture, view: *mut sfView) -> IntRect;
    pub fn sfRenderTexture_mapPixelToCoords(renderTexture: *mut sfRenderTexture, point: Vector2i, view: *mut sfView) -> Vector2f;
    pub fn sfRenderTexture_mapCoordsToPixel(renderTexture: *mut sfRenderTexture, point: Vector2f, view: *mut sfView) -> Vector2i;
    pub fn sfRenderTexture_drawSprite(renderTexture: *mut sfRenderTexture, object: *mut sfSprite, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawText(renderTexture: *mut sfRenderTexture, object: *mut sfText, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawShape(renderTexture: *mut sfRenderTexture, object: *mut sfShape, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawCircleShape(renderTexture: *mut sfRenderTexture, object: *mut sfCircleShape, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawConvexShape(renderTexture: *mut sfRenderTexture, object: *mut sfConvexShape, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawRectangleShape(renderTexture: *mut sfRenderTexture, object: *mut sfRectangleShape, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawVertexArray(renderTexture: *mut sfRenderTexture, object: *mut sfVertexArray, states: *mut sfRenderStates);
    pub fn sfRenderTexture_drawPrimitives(renderTexture: *mut sfRenderTexture, vertices: *const sfVertex, vertexCount: c_uint, ttype: sfPrimitiveType, states: *mut sfRenderStates);
    pub fn sfRenderTexture_pushGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_popGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_resetGLStates(renderTexture: *mut sfRenderTexture);
    pub fn sfRenderTexture_getTexture(renderTexture: *mut sfRenderTexture) -> *mut sfTexture;
    pub fn sfRenderTexture_setSmooth(renderTexture: *mut sfRenderTexture, smooth: sfBool);
    pub fn sfRenderTexture_isSmooth(renderTexture: *mut sfRenderTexture) -> sfBool;
}

pub enum sfShape {}

extern "C" {
    pub fn sfShape_create(getPointCount: extern "C" fn(*mut c_void) -> u32, getPoint: extern "C" fn(u32, *mut c_void) -> Vector2f, userData: *mut c_void) -> *mut sfShape;
    pub fn sfShape_destroy(shape: *mut sfShape);
    pub fn sfShape_setPosition(shape: *mut sfShape, position: Vector2f);
    pub fn sfShape_setRotation(shape: *mut sfShape, angle: c_float);
    pub fn sfShape_setScale(shape: *mut sfShape, scale: Vector2f);
    pub fn sfShape_setOrigin(shape: *mut sfShape, origin: Vector2f);
    pub fn sfShape_getPosition(shape: *mut sfShape) -> Vector2f;
    pub fn sfShape_getRotation(shape: *mut sfShape) -> c_float;
    pub fn sfShape_getScale(shape: *mut sfShape) -> Vector2f;
    pub fn sfShape_getOrigin(shape: *mut sfShape) -> Vector2f;
    pub fn sfShape_move(shape: *mut sfShape, offset: Vector2f);
    pub fn sfShape_rotate(shape: *mut sfShape, angle: c_float);
    pub fn sfShape_scale(shape: *mut sfShape, factors: Vector2f);
    pub fn sfShape_getTransform(shape: *mut sfShape) -> sfTransform;
    pub fn sfShape_getInverseTransform(shape: *mut sfShape) -> sfTransform;
    pub fn sfShape_setTexture(shape: *mut sfShape, texture: *mut sfTexture, reset_rect: sfBool);
    pub fn sfShape_setTextureRect(shape: *mut sfShape, rect: IntRect);
    pub fn sfShape_setFillColor(shape: *mut sfShape, color: sfColor);
    pub fn sfShape_setOutlineColor(shape: *mut sfShape, color: sfColor);
    pub fn sfShape_setOutlineThickness(shape: *mut sfShape, thickness: c_float);
    pub fn sfShape_getTexture(shape: *mut sfShape) -> *mut sfTexture;
    pub fn sfShape_getTextureRect(shape: *mut sfShape) -> IntRect;
    pub fn sfShape_getFillColor(shape: *mut sfShape) -> sfColor;
    pub fn sfShape_getOutlineColor(shape: *mut sfShape) -> sfColor;
    pub fn sfShape_getOutlineThickness(shape: *mut sfShape) -> c_float;
    pub fn sfShape_getPointCount(shape: *mut sfShape) -> c_uint;
    pub fn sfShape_getPoint(shape: *mut sfShape, index: c_uint) -> Vector2f;
    pub fn sfShape_getLocalBounds(shape: *mut sfShape) -> FloatRect;
    pub fn sfShape_getGlobalBounds(shape: *mut sfShape) -> FloatRect;
    pub fn sfShape_update(shape: *mut sfShape);
}

pub enum sfSprite {}

extern "C" {
    pub fn sfSprite_create() -> *mut sfSprite;
    pub fn sfSprite_copy(sprite: *mut sfSprite) -> *mut sfSprite;
    pub fn sfSprite_destroy(sprite: *mut sfSprite);
    pub fn sfSprite_setPosition(sprite: *mut sfSprite, position: Vector2f);
    pub fn sfSprite_setRotation(sprite: *mut sfSprite, angle: c_float);
    pub fn sfSprite_setScale(sprite: *mut sfSprite, scale: Vector2f);
    pub fn sfSprite_setOrigin(sprite: *mut sfSprite, origin: Vector2f);
    pub fn sfSprite_getPosition(sprite: *mut sfSprite) -> Vector2f;
    pub fn sfSprite_getRotation(sprite: *mut sfSprite) -> c_float;
    pub fn sfSprite_getScale(sprite: *mut sfSprite) -> Vector2f;
    pub fn sfSprite_getOrigin(sprite: *mut sfSprite) -> Vector2f;
    pub fn sfSprite_move(sprite: *mut sfSprite, offset: Vector2f);
    pub fn sfSprite_rotate(sprite: *mut sfSprite, angle: c_float);
    pub fn sfSprite_scale(sprite: *mut sfSprite, factors: Vector2f);
    pub fn sfSprite_getTransform(sprite: *mut sfSprite) -> sfTransform;
    pub fn sfSprite_getInverseTransform(sprite: *mut sfSprite) -> sfTransform;
    pub fn sfSprite_setTexture(sprite: *mut sfSprite, texture: *mut sfTexture, reset_rect: sfBool);
    pub fn sfSprite_setTextureRect(sprite: *mut sfSprite, rectangle: IntRect);
    pub fn sfSprite_setColor(sprite: *mut sfSprite, color: sfColor);
    pub fn sfSprite_getTexture(sprite: *mut sfSprite) -> *mut sfTexture;
    pub fn sfSprite_getTextureRect(sprite: *mut sfSprite) -> IntRect;
    pub fn sfSprite_getColor(sprite: *mut sfSprite) -> sfColor;
    pub fn sfSprite_getLocalBounds(sprite: *mut sfSprite) -> FloatRect;
    pub fn sfSprite_getGlobalBounds(sprite: *mut sfSprite) -> FloatRect;
}

pub enum sfText {}

extern "C" {
    pub fn sfText_create() -> *mut sfText;
    pub fn sfText_copy(text: *mut sfText) -> *mut sfText;
    pub fn sfText_destroy(text: *mut sfText);
    pub fn sfText_setPosition(text: *mut sfText, position: Vector2f);
    pub fn sfText_setRotation(text: *mut sfText, angle: c_float);
    pub fn sfText_setScale(text: *mut sfText, scale: Vector2f);
    pub fn sfText_setOrigin(text: *mut sfText, origin: Vector2f);
    pub fn sfText_getPosition(text: *mut sfText) -> Vector2f;
    pub fn sfText_getRotation(text: *mut sfText) -> c_float;
    pub fn sfText_getScale(text: *mut sfText) -> Vector2f;
    pub fn sfText_getOrigin(text: *mut sfText) -> Vector2f;
    pub fn sfText_move(text: *mut sfText, offset: Vector2f);
    pub fn sfText_rotate(text: *mut sfText, angle: c_float);
    pub fn sfText_scale(text: *mut sfText, factors: Vector2f);
    pub fn sfText_getTransform(text: *mut sfText) -> sfTransform;
    pub fn sfText_getInverseTransform(text: *mut sfText) -> sfTransform;
    pub fn sfText_setString(text: *mut sfText, string: *const c_char);
    pub fn sfText_setUnicodeString(text: *mut sfText, string: *const u32 );
    pub fn sfText_setFont(text: *mut sfText, font: *mut sfFont);
    pub fn sfText_setCharacterSize(text: *mut sfText, size: c_uint);
    pub fn sfText_setStyle(text: *mut sfText, style: u32);
    pub fn sfText_setColor(text: *mut sfText, color: sfColor);
    pub fn sfText_getString(text: *mut sfText) -> *const c_char;
    pub fn sfText_getUnicodeString(text: *mut sfText) -> *const u32;
    pub fn sfText_getFont(text: *mut sfText) -> *mut sfFont;
    pub fn sfText_getCharacterSize(text: *mut sfText) -> c_uint;
    pub fn sfText_getStyle(text: *mut sfText) -> u32;
    pub fn sfText_getColor(text: *mut sfText) -> sfColor;
    pub fn sfText_findCharacterPos(text: *mut sfText, index: size_t) -> Vector2f;
    pub fn sfText_getLocalBounds(text: *mut sfText) -> FloatRect;
    pub fn sfText_getGlobalBounds(text: *mut sfText) -> FloatRect;
}

pub enum sfTexture {}

extern "C" {
    pub fn sfTexture_create(width: c_uint, height: c_uint) -> *mut sfTexture;
    pub fn sfTexture_createFromFile(filename: *mut c_char, area: *const IntRect) -> *mut sfTexture;
    pub fn sfTexture_createFromMemory(data: *const c_uchar, sizeInBytes: size_t , area: *const IntRect) -> *mut sfTexture;
    pub fn sfTexture_createFromStream(stream: *mut sfInputStream, area: *mut IntRect) -> *mut sfTexture;
    pub fn sfTexture_createFromImage(image :*mut sfImage, area: *const IntRect) -> *mut sfTexture;
    pub fn sfTexture_copy(texture: *mut sfTexture) -> *mut sfTexture;
    pub fn sfTexture_destroy(texture: *mut sfTexture);
    pub fn sfTexture_getSize(texture: *mut sfTexture) -> Vector2u;
    pub fn sfTexture_copyToImage(texture: *mut sfTexture) -> *mut sfImage;
    pub fn sfTexture_updateFromPixels(texture: *mut sfTexture, pixels: *const u8, width: c_uint, height: c_uint, x: c_uint, y: c_uint);
    pub fn sfTexture_updateFromImage(texture: *mut sfTexture, image: *mut sfImage, x: c_uint, y: c_uint);
    pub fn sfTexture_updateFromWindow(texture: *mut sfTexture, window: *mut sfWindow, x: c_uint, y: c_uint);
    pub fn sfTexture_updateFromRenderWindow(texture: *mut sfTexture, renderWindow: *mut sfRenderWindow, x: c_uint, y: c_uint);
    pub fn sfTexture_setSmooth(texture: *mut sfTexture, smooth: sfBool);
    pub fn sfTexture_isSmooth(texture: *mut sfTexture) -> sfBool;
    pub fn sfTexture_setRepeated(texture: *mut sfTexture, repeated: sfBool);
    pub fn sfTexture_isRepeated(texture: *mut sfTexture) -> sfBool;
    pub fn sfTexture_bind(texture: *mut sfTexture);
    pub fn sfTexture_getMaximumSize() -> c_uint;
}

extern "C" {
    pub fn sfTransform_fromMatrix(a01: f32, a02: f32, a03: f32, b01: f32, b02: f32, b03: f32, c01: f32, c02: f32, c03: f32) -> sfTransform;
    pub fn sfTransform_getMatrix(tranform: *mut sfTransform, matrix: *mut f32);
    pub fn sfTransform_getInverse(transform: *mut sfTransform) -> sfTransform;
    pub fn sfTransform_transformPoint(transform: *mut sfTransform, point: Vector2f) -> Vector2f;
    pub fn sfTransform_transformRect(transform: *mut sfTransform, rectangle: FloatRect) -> FloatRect;
    pub fn sfTransform_combine(transform: *mut sfTransform, other: *mut sfTransform);
    pub fn sfTransform_translate(transform: *mut sfTransform, x: c_float, y: c_float);
    pub fn sfTransform_rotate(transform: *mut sfTransform, angle: c_float);
    pub fn sfTransform_rotateWithCenter(transform: *mut sfTransform, angle: c_float, center_x: c_float, center_y: c_float);
    pub fn sfTransform_scale(transform: *mut sfTransform, scale_x: c_float, scale_y: c_float);
    pub fn sfTransform_scaleWithCenter(transform: *mut sfTransform, scale_x: c_float, scale_y: c_float, center_x: c_float, center_y: c_float);
}

pub enum sfTransformable {}

extern "C" {
    pub fn sfTransformable_create() -> *mut sfTransformable;
    pub fn sfTransformable_copy(transformable: *mut sfTransformable) -> *mut sfTransformable;
    pub fn sfTransformable_destroy(transformable: *mut sfTransformable);
    pub fn sfTransformable_setPosition(transformable: *mut sfTransformable, position: Vector2f);
    pub fn sfTransformable_setRotation(transformable: *mut sfTransformable, angle: c_float);
    pub fn sfTransformable_setScale(transformable: *mut sfTransformable, scale: Vector2f);
    pub fn sfTransformable_setOrigin(transformable: *mut sfTransformable, origin: Vector2f);
    pub fn sfTransformable_getPosition(transformable: *mut sfTransformable) -> Vector2f;
    pub fn sfTransformable_getRotation(transformable: *mut sfTransformable) -> c_float;
    pub fn sfTransformable_getScale(transformable: *mut sfTransformable) -> Vector2f;
    pub fn sfTransformable_getOrigin(transformable: *mut sfTransformable) -> Vector2f;
    pub fn sfTransformable_move(transformable: *mut sfTransformable, offset: Vector2f);
    pub fn sfTransformable_rotate(transformable: *mut sfTransformable, angle: c_float);
    pub fn sfTransformable_scale(transformable: *mut sfTransformable, factors: Vector2f);
    pub fn sfTransformable_getTransform(transformable: *mut sfTransformable) -> sfTransform;
    pub fn sfTransformable_getInverseTransform(transformable: *mut sfTransformable) -> sfTransform;
}

pub enum sfVertexArray {}

extern "C" {
    pub fn sfVertexArray_create() -> *mut sfVertexArray;
    pub fn sfVertexArray_copy(vertexArray: *mut sfVertexArray) -> *mut sfVertexArray;
    pub fn sfVertexArray_destroy(vertexArray: *mut sfVertexArray);
    pub fn sfVertexArray_getVertexCount(vertexArray: *mut sfVertexArray) -> c_uint;
    pub fn sfVertexArray_getVertex(vertexArray: *mut sfVertexArray, index: c_uint) -> *mut sfVertex;
    pub fn sfVertexArray_clear(vertexArray: *mut sfVertexArray);
    pub fn sfVertexArray_resize(vertexArray: *mut sfVertexArray, vertexCount: c_uint);
    pub fn sfVertexArray_append(vertexArray: *mut sfVertexArray, vertex: sfVertex);
    pub fn sfVertexArray_setPrimitiveType(vertexArray: *mut sfVertexArray, stype: sfPrimitiveType);
    pub fn sfVertexArray_getPrimitiveType(vertexArray: *mut sfVertexArray) -> sfPrimitiveType;
    pub fn sfVertexArray_getBounds(vertexArray: *mut sfVertexArray) -> FloatRect;
}

pub enum sfView {}

extern "C" {
    pub fn sfView_create() -> *mut sfView;
    pub fn sfView_createFromRect(rectangle: FloatRect) -> *mut sfView;
    pub fn sfView_copy(view: *mut sfView) -> *mut sfView;
    pub fn sfView_destroy(view: *mut sfView);
    pub fn sfView_setCenter(view: *mut sfView, center: Vector2f);
    pub fn sfView_setSize(view: *mut sfView, size: Vector2f);
    pub fn sfView_setRotation(view: *mut sfView, angle: c_float);
    pub fn sfView_setViewport(view: *mut sfView, viewport: FloatRect);
    pub fn sfView_reset(view: *mut sfView, rectangle: FloatRect);
    pub fn sfView_getCenter(view: *mut sfView) -> Vector2f;
    pub fn sfView_getSize(view: *mut sfView) -> Vector2f;
    pub fn sfView_getRotation(view: *mut sfView) -> c_float;
    pub fn sfView_getViewport(view: *mut sfView) -> FloatRect;
    pub fn sfView_move(view: *mut sfView, offset: Vector2f);
    pub fn sfView_rotate(view: *mut sfView, angle: c_float);
    pub fn sfView_zoom(view: *mut sfView, factor: c_float);
}
