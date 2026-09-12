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

def main():
    docx_dir = 'docs/conversation_docx/New code language_'
    files = sorted(glob.glob(os.path.join(docx_dir, '*.docx')))
    
    queries = [
        "VTABLE", "Static Trait",
        "RAW hazard",
        "Hardware Trap",
        "LFSR",
        "Double-Buffering",
        "CYCLE_CNT",
        "official ABI"
    ]
    
    for q in queries:
        print(f"\n==================================================")
        print(f"   SEARCH QUERY: {q}")
        print(f"==================================================")
        matched_files = [f for f in files if q.lower() in os.path.basename(f).lower()]
        for f in matched_files:
            print(f"\nFILE: {os.path.basename(f)}")
            text = extract_text(f)
            # print up to 30 lines
            lines = [l.strip() for l in text.split('\n') if l.strip()]
            for l in lines[:35]:
                print(f"  {l}")

if __name__ == '__main__':
    main()
