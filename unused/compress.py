import zipapp
from pathlib import Path
import zipfile


zipapp.create_archive(source=Path('./__main__.py'), interpreter='/usr/bin/python3', compressed=True, target=Path('main.pyz'))