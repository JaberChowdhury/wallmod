import re

with open("gowall_src/internal/image/themes.go", "r") as f:
    content = f.read()

themes = {}
current_theme = None
colors = []
for line in content.splitlines():
    m = re.match(r'\s*Name:\s*"([^"]+)",', line)
    if m:
        current_theme = m.group(1)
        colors = []
    
    if current_theme:
        rgba = re.search(r'color\.RGBA\{R:\s*(\d+),\s*G:\s*(\d+),\s*B:\s*(\d+)', line)
        if rgba:
            colors.append(f"[{rgba.group(1)}, {rgba.group(2)}, {rgba.group(3)}]")
        
        if line.strip() == "},":
            if colors:
                themes[current_theme] = ", ".join(colors)
            current_theme = None

for name, cols in themes.items():
    print(f'        "{name}" => vec![{cols}],')

