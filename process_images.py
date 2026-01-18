#!/usr/bin/env python3
"""
统一的图片处理脚本
功能：JPG转PNG、转RGBA、移除白色背景
"""

from PIL import Image
import os

IMAGES_DIR = os.path.join(os.path.dirname(__file__), 'images')

def convert_jpg_to_png():
    """将所有 JPG 转换为 PNG"""
    print("1️⃣  转换 JPG → PNG...")
    for f in os.listdir(IMAGES_DIR):
        if f.lower().endswith('.jpg'):
            jpg_path = os.path.join(IMAGES_DIR, f)
            png_name = f[:-4] + '.png'
            png_path = os.path.join(IMAGES_DIR, png_name)
            
            img = Image.open(jpg_path)
            img.save(png_path)
            print(f"   ✓ {f} → {png_name}")

def convert_to_rgba():
    """转换所有 PNG 为 RGBA 格式"""
    print("\n2️⃣  转换格式为 RGBA...")
    for f in os.listdir(IMAGES_DIR):
        if f.lower().endswith('.png'):
            png_path = os.path.join(IMAGES_DIR, f)
            img = Image.open(png_path)
            
            if img.mode != 'RGBA':
                img = img.convert('RGBA')
                img.save(png_path)
                print(f"   ✓ {f} 已转为 RGBA")
            else:
                print(f"   ✓ {f} 已是 RGBA")

def remove_white_background(threshold=150, tolerance=50):
    """
    移除图片中的白色背景
    
    Args:
        threshold: RGB 平均值大于此值认为是白色（0-255）
        tolerance: 三个通道的差异容差（0-255）
    """
    print(f"\n3️⃣  移除白色背景（阈值={threshold}, 容差={tolerance})...")
    
    mole_images = ['mole.png', 'helmet_mole.png', 'kobe.png']
    
    for img_file in mole_images:
        img_path = os.path.join(IMAGES_DIR, img_file)
        if not os.path.exists(img_path):
            print(f"   ✗ {img_file} 不存在")
            continue
        
        img = Image.open(img_path)
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        
        pixels = img.load()
        width, height = img.size
        
        # 处理每个像素
        for y in range(height):
            for x in range(width):
                r, g, b, a = pixels[x, y]
                
                # 计算平均值
                avg = (int(r) + int(g) + int(b)) / 3
                
                # 检查是否接近白色
                is_white = (
                    avg > threshold and
                    abs(int(r) - int(g)) < tolerance and
                    abs(int(g) - int(b)) < tolerance and
                    abs(int(r) - int(b)) < tolerance
                )
                
                if is_white:
                    pixels[x, y] = (r, g, b, 0)  # 设为透明
        
        img.save(img_path)
        print(f"   ✓ {img_file} 背景已移除")

def main():
    print("=" * 50)
    print("图片处理工具")
    print("=" * 50)
    
    try:
        convert_jpg_to_png()
        convert_to_rgba()
        remove_white_background(threshold=150, tolerance=50)
        
        print("\n" + "=" * 50)
        print("✅ 所有处理完成！")
        print("=" * 50)
        
    except Exception as e:
        print(f"\n❌ 错误: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    main()
