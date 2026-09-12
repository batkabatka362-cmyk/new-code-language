import os
import glob
import zipfile
import xml.etree.ElementTree as ET
import sys

sys.stdout.reconfigure(encoding='utf-8')

def extract_text(docx_path):
    try:
        with zipfile.ZipFile(docx_path) as z:
            xml_content = z.read('word/document.xml')
            tree = ET.fromstring(xml_content)
            paragraphs = []
            for p in tree.iter('{http://schemas.openxmlformats.org/wordprocessingml/2006/main}p'):
                texts = [node.text for node in p.iter('{http://schemas.openxmlformats.org/wordprocessingml/2006/main}t') if node.text]
                if texts:
                    paragraphs.append(''.join(texts))
            return '\n'.join(paragraphs)
    except Exception as e:
        return ""

docx_dir = 'docs/conversation_docx/New code language_'
files = sorted(glob.glob(os.path.join(docx_dir, '*.docx')))

targets = [
    ("Trait", "VTABLE-гүй Static Trait"),
    ("RAW Hazard", "RAW hazard"),
    ("Hardware Trap", "0-д хуваах"),
    ("DMA Double Buffering", "Double-Buffering"),
]

for title, q in targets:
    print(f"\n==================================================")
    print(f"   TARGET: {title} ({q})")
    print(f"==================================================")
    for f in files:
        if q.lower() in os.path.basename(f).lower():
            text = extract_text(f)
            lines = [l.strip() for l in text.split('\n') if l.strip()]
            print(f"FILE: {os.path.basename(f)}")
            for l in lines[:25]:
                # filter long code dumps
                if len(l) < 200:
                    print(f"  {l}")
            break
