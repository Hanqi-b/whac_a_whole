#!/usr/bin/env python3
"""
图片蓝幕处理脚本
功能：
- 遍历 images 文件夹的所有图片
- 对 1024×1024 的图片，移除蓝幕背景（变透明）
- 缩放到 256×256
- 保存为新文件（原文件保留）
"""

from PIL import Image
import os
import colorsys

IMAGES_DIR = os.path.join(os.path.dirname(__file__), '..', 'images')

def is_blue_color(r, g, b, hue_range=(200, 260), saturation_min=0.3):
    """
    判断颜色是否为蓝色
    使用 HSV 色彩空间判断蓝幕
    
    Args:
        r, g, b: RGB 值（0-255）
        hue_range: 蓝色色相范围（度数，0-360）
        saturation_min: 最小饱和度（0-1）
    """
    # 标准化 RGB
    r_norm, g_norm, b_norm = r / 255.0, g / 255.0, b / 255.0
    
    # 转换为 HSV
    h, s, v = colorsys.rgb_to_hsv(r_norm, g_norm, b_norm)
    
    # H 值转换为度数（0-360）
    hue_deg = h * 360
    
    # 判断是否在蓝色范围内
    is_blue_hue = hue_range[0] <= hue_deg <= hue_range[1]
    is_saturated = s >= saturation_min
    
    return is_blue_hue and is_saturated

def remove_blue_screen(img_path, output_path):
    """
    移除蓝幕，缩放图片
    
    Args:
        img_path: 输入图片路径
        output_path: 输出图片路径
    """
    img = Image.open(img_path)
    
    # 检查尺寸
    if img.size != (1024, 1024):
        return False
    
    print(f"   处理: {os.path.basename(img_path)} ({img.size})")
    
    # 转换为 RGBA
    if img.mode != 'RGBA':
        img = img.convert('RGBA')
    
    # 处理像素
    pixels = img.load()
    width, height = img.size
    
    removed_count = 0
    for y in range(height):
        for x in range(width):
            r, g, b, a = pixels[x, y]
            
            # 检测蓝色像素
            if is_blue_color(r, g, b):
                pixels[x, y] = (r, g, b, 0)  # 设为透明
                removed_count += 1
    
    print(f"      ✓ 移除蓝色像素: {removed_count} 个")
    
    # 缩放到 256×256
    img_resized = img.resize((256, 256), Image.Resampling.LANCZOS)
    
    # 保存新文件
    img_resized.save(output_path)
    print(f"      ✓ 已保存: {os.path.basename(output_path)} (256×256)")
    
    return True

def main():
    print("=" * 60)
    print("蓝幕背景处理工具")
    print("=" * 60)
    print(f"扫描目录: {IMAGES_DIR}\n")
    
    processed_count = 0
    skipped_count = 0
    
    for filename in os.listdir(IMAGES_DIR):
        if not filename.lower().endswith(('.png', '.jpg', '.jpeg')):
            continue
        
        img_path = os.path.join(IMAGES_DIR, filename)
        
        try:
            # 生成输出文件名
            name, ext = os.path.splitext(filename)
            output_filename = f"{name}_processed.png"  # 始终输出为 PNG
            output_path = os.path.join(IMAGES_DIR, output_filename)
            
            # 处理图片
            if remove_blue_screen(img_path, output_path):
                processed_count += 1
            else:
                print(f"   跳过: {filename} (不是 1024×1024)")
                skipped_count += 1
                
        except Exception as e:
            print(f"   ❌ 错误: {filename} - {e}")
    
    print("\n" + "=" * 60)
    print(f"✅ 完成！")
    print(f"   处理: {processed_count} 张")
    print(f"   跳过: {skipped_count} 张")
    print("=" * 60)

if __name__ == '__main__':
    main()
