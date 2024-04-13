import os
import requests
import random
from bs4 import BeautifulSoup
from pathlib import Path

default_counts={'train': 50, 'test': 5}

def create_directories(base_path, languages, types=('train', 'test'), counts=default_counts):
    for t in types:
        for lang in languages:
            dir_path = Path(base_path) / t / lang
            dir_path.mkdir(parents=True, exist_ok=True)
            fill_directory(dir_path, counts[t], lang)


def fill_directory(directory, count, language):
    texts = fetch_texts(language, count)
    for i, text in enumerate(texts):
        file_path = directory / f"{i+1}.txt"
        with open(file_path, 'w', encoding='utf-8') as file:
            file.write(text)


def fetch_texts(language, count):
    """
    Fetches a specified number of random Wikipedia articles in a given language.
    """
    lang_codes = {
        'English': 'en',
        'German': 'de',
        'Polish': 'pl'
    }
    base_url = f"https://{lang_codes[language]}.wikipedia.org/w/api.php"
    texts = []

    for _ in range(count):
        params = {
            'action': 'query',
            'format': 'json',
            'generator': 'random',
            'grnnamespace': 0,  # 0 limits to articles only
            'grnlimit': 1,      # Fetch one random article at a time
            'prop': 'extracts',
            'explaintext': True,
            'exsectionformat': 'plain'
        }

        response = requests.get(base_url, params=params)
        data = response.json()
        
        pages = data.get('query', {}).get('pages')
        if pages:
            for page_id in pages:
                extract = pages[page_id].get('extract')
                if extract:
                    texts.append(extract)
                break  # Only need one extract per loop iteration

    return texts


if __name__ == "__main__":
    languages = ['English', 'German', 'Polish']
    base_path = '.'
    create_directories(base_path, languages)

