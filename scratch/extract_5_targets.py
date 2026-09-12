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
    "VTABLE-гүй Static Trait",
    "RAW hazard",
    "0-д хуваах",
    "LFSR Random",
    "Double-Buffering"
]

for t in targets:
    print(f"\n==================================================")
    print(f"   TARGET: {t}")
    print(f"==================================================")
    for f in files:
        if t.lower() in os.path.basename(f).lower():
            text = extract_text(f)
            lines = [l.strip() for l in text.split('\n') if l.strip()]
            print(f"FILE: {os.path.basename(f)}")
            for l in lines[:40]:
                print(f"  {l}")
            break
