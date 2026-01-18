from PIL import Image
import os

# 图片文件夹路径
images_dir = os.path.join(os.path.dirname(__file__), 'images')

# 获取所有图片文件
image_files = [f for f in os.listdir(images_dir) if f.lower().endswith(('.jpg', '.jpeg', '.png'))]

print(f"找到 {len(image_files)} 张图片")

# 处理每张图片
for idx, img_file in enumerate(image_files, 1):
    img_path = os.path.join(images_dir, img_file)
    
    try:
        # 打开图片
        img = Image.open(img_path)
        print(f"[{idx}] 处理: {img_file} (原始大小: {img.size})")
        
        # 裁剪左上角 300x300 区域
        # crop 参数: (left, top, right, bottom)
        cropped = img.crop((0, 0, 300, 300))
        
        # 生成新文件名
        name, ext = os.path.splitext(img_file)
        output_name = f"{name}_cropped{ext}"
        output_path = os.path.join(images_dir, output_name)
        
        # 保存裁剪后的图片
        cropped.save(output_path)
        print(f"    ✓ 已保存: {output_name}")
        
    except Exception as e:
        print(f"    ✗ 错误: {e}")

print("\n完成！")
