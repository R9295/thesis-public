import sys
import glob
import subprocess


def main():
    args = sys.argv
    if len(args) < 2:
        print('usage: ./get_results.py <folder_path>')
        return
    folder = sys.argv[1]
    trials = glob.glob(f'{folder}/**/trial-*', recursive=True)
    if len(trials) == 0:
        print('no trials found?')
        return
    for i in trials:
        folder_name = i.split('/')[-1]
        print(f'doing {folder_name}')
        snapshots = glob.glob(f'{i}/corpus/*.tar.gz')
        for item in snapshots:
            item_name = item.split('/')[-1]
            print(f'    extracting {item_name}')
            subprocess.call(['tar', '-xvf', item_name], cwd=f'{i}/corpus')

if __name__ == '__main__':
    sys.exit(main())

