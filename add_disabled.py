import re
import sys

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Find where to insert `let is_loading = view.app.state.is_loading();`
    if 'let is_loading = ' not in content:
        # insert at the beginning of the render function
        content = re.sub(
            r'(pub fn render_[a-z]+\(view:\s*&mut\s*(?:crate::ui::)?WallmodView,\s*cx:\s*&mut\s*ViewContext<WallmodView>\)\s*->\s*impl\s*IntoElement\s*\{)',
            r'\1\n    let is_loading = view.app.state.is_loading();',
            content
        )

    # We need a robust way to match Button::new(...) and insert .disabled(is_loading)
    # We can use a simple state machine or regex.
    # Button::new("something"), Button::new(SharedString::from(...))
    # A simple regex for Button::new(something without parens) is easy, but with parens it's hard.
    # Let's just find Button::new and find the matching closing parenthesis.
    
    def insert_disabled(text, component_name):
        idx = 0
        while True:
            idx = text.find(component_name + "::new(", idx)
            if idx == -1:
                break
            # find matching parenthesis
            open_count = 0
            end_idx = idx + len(component_name + "::new(") - 1
            for i in range(end_idx, len(text)):
                if text[i] == '(':
                    open_count += 1
                elif text[i] == ')':
                    open_count -= 1
                    if open_count == 0:
                        # insert .disabled(is_loading)
                        insert_pos = i + 1
                        text = text[:insert_pos] + ".disabled(is_loading)" + text[insert_pos:]
                        idx = insert_pos + len(".disabled(is_loading)")
                        break
        return text

    content = insert_disabled(content, "Button")
    content = insert_disabled(content, "Slider")
    content = insert_disabled(content, "Switch")

    with open(filepath, 'w') as f:
        f.write(content)

for file in ["src/ui/sidebar.rs", "src/ui/header.rs", "src/ui/workspace.rs"]:
    process_file(file)

print("Done")
