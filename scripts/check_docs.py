"""Non-mutating specification-link and preview-status checks."""
from pathlib import Path
import re
ROOT = Path(__file__).resolve().parents[1]
for file in ROOT.rglob('*.md'):
    if '.git' in file.parts:
        continue
    text = file.read_text(encoding='utf-8')
    if sum(line.startswith('```') for line in text.splitlines()) % 2:
        raise ValueError(f'Unclosed example fence: {file.relative_to(ROOT)}')
    for link in re.findall(r'\]\(([^)\s]+)\)', text):
        if link.startswith(('https:', 'http:', 'mailto:', '#')):
            continue
        target = (file.parent / link.split('#')[0]).resolve()
        if not target.is_relative_to(ROOT) or not target.exists():
            raise ValueError(f'Invalid local documentation link: {file.relative_to(ROOT)}: {link}')
assert 'no working compiler release yet' in (ROOT / 'README.md').read_text()
assert 'Apache License' in (ROOT / 'LICENSE').read_text()
print('Specification links, examples and preview boundary verified')
