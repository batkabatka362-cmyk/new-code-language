import json
import glob
import os

def main():
    with open('docs/ocr_data/docx_milestones.json', encoding='utf-8') as f:
        milestones = json.load(f)
    print(f"Total milestones: {len(milestones)}")
    
    # Check topics mentioned across milestones
    topics = {}
    for m in milestones:
        for match in m.get('matches', []):
            topics[match] = topics.get(match, 0) + 1
    print("\nTopic occurrences in docx milestones:")
    for k, v in sorted(topics.items(), key=lambda x: -x[1]):
        print(f"  {k}: {v}")
        
    print("\nSample milestone file titles:")
    for m in milestones[:30]:
        print(f"  [{m['index']}] {m['file']}")

if __name__ == '__main__':
    main()
