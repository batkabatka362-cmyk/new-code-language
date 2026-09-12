import os
import glob
import zipfile
import xml.etree.ElementTree as ET
import sys

sys.stdout.reconfigure(encoding='utf-8')

def extract_text_from_docx(docx_path):
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

def main():
    docx_dir = 'docs/conversation_docx/New code language_'
    docx_files = sorted(glob.glob(os.path.join(docx_dir, '*.docx')))
    print(f"Found {len(docx_files)} docx files.")

    # Let's inspect the files with explicit milestone titles
    specific_keywords = [
        "1 дэх чиглэл", "2 дахь чиглэл", "3 дахь чиглэл", "4 дэх чиглэл",
        "чиглэл", "тархи", "алхам", "RTL", "Driver", "Trap", "Hazard",
        "Sub-Byte", "Static Trait", "VTABLE", "LFSR", "DMA", "ABI", "Python",
        "CYCLE_CNT", "STALL_CNT"
    ]

    interesting_files = []
    for f in docx_files:
        basename = os.path.basename(f)
        for kw in specific_keywords:
            if kw.lower() in basename.lower():
                interesting_files.append(f)
                break

    print(f"Found {len(interesting_files)} key architectural milestone files:\n")
    for f in interesting_files:
        print(f"--- {os.path.basename(f)} ---")
        text = extract_text_from_docx(f)
        lines = [l.strip() for l in text.split('\n') if l.strip()]
        # Print first 5-8 informative lines
        for l in lines[:10]:
            if len(l) > 10:
                print(f"    {l[:120]}")
        print()

if __name__ == '__main__':
    main()
