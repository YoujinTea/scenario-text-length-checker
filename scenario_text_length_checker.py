import os
import json
from pathlib import Path
import re


def get_settings():
    # settings.jsonが存在しない場合は作成する
    if not Path('settings.json').exists():
        settings = {
            'linefeedTag': ['[r]'],
            'pageBreakTag': ['[p]'],
            'lineCount': 2,
            'maxRowLength': 30
        }
        with open('settings.json', 'w', encoding='utf-8') as f:
            json.dump(settings, f, ensure_ascii=False, indent=4)

        return settings

    # settings.jsonが存在する場合は読み込む
    with open('settings.json', 'r', encoding='utf-8') as f:
        settings = json.load(f)
        return settings


def search_ks_files(directory):
    ks_files = []

    for path in directory.iterdir():
        # ディレクトリの場合は再帰的に探索
        if path.is_dir():
            ks_files.extend(search_ks_files(path))

        # .ksファイルの場合はリストに追加
        elif path.suffix == '.ks':
            ks_files.append(path)

    return ks_files


def check_statement_length(statement, settings):
    current_line = 0
    for s in statement:
        length = len(s)
        while length > 0:
            length -= settings['maxRowLength']
            current_line += 1

    return current_line > settings['lineCount']


def check_text_length(ks_file, display_path, settings):
    with open(ks_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    before_statement = ['']
    for i, line in enumerate(lines):
        line = line.strip()

        # コメント行はスキップ
        if line.startswith(';'):
            continue

        statements = [line]

        # 改ページタグで区切る
        for tag in settings['pageBreakTag']:
            statements = [s.split(tag) for s in statements]
            statements = [x for row in statements for x in row]

        def split_line_callback(statement):
            statement_tmp = [statement.strip()]

            # 改行タグで区切る
            for tag in settings['linefeedTag']:
                statement_tmp = [s.split(tag) for s in statement_tmp]
                statement_tmp = [x for row in statement_tmp for x in row]

            # 文章を整形
            statement_tmp = [s.strip() for s in statement_tmp]

            # 文章の中のタグを削除
            statement_tmp = [re.sub(r'\[.*?\]', '', s) for s in statement_tmp]

            return statement_tmp

        statements = list(map(lambda x: split_line_callback(x), statements))

        # 空行はスキップ
        if len(statements) == 0:
            continue

        # 前行の文章を結合
        statements[0][-1] = before_statement[-1] + statements[0][-1]

        # 行数が設定値を超えているかチェック
        for statement in statements[0:-1]:
            if check_statement_length(statement, settings):
                print(f'{display_path} {i + 1}行: 文章が長すぎます。')

        # 最後の文章を次の行に引き継ぐ
        before_statement = statements[-1]


def main():
    path = Path()

    if path.resolve().parent.name != 'others':
        print('Error: このアプリケーションはothersディレクトリに配置してください。')
        os.system('PAUSE')
        return

    scenario_dir = path.resolve().parent.parent / 'scenario'

    if not scenario_dir.exists():
        print('Error: scenarioディレクトリが存在しません。')
        os.system('PAUSE')
        return

    settings = get_settings()

    ks_files = search_ks_files(path.resolve().parent.parent / 'scenario')

    for ks_file in ks_files:
        check_text_length(ks_file, ks_file.relative_to(scenario_dir), settings)
    
    os.system('PAUSE')

if __name__ == "__main__":
    main()
