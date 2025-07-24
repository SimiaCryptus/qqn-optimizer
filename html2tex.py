from bs4 import BeautifulSoup
import re
import argparse
import sys

def clean_html_to_latex(html_file, output_file='tables.tex', caption_prefix='Experimental Results Table', label_prefix='tab:results'):
    with open(html_file, 'r') as f:
        content = f.read()

    soup = BeautifulSoup(content, 'html.parser')

    # Extract tables and convert to LaTeX
    tables = soup.find_all('table')
    latex_content = []

    for i, table in enumerate(tables):
        rows = []
        for tr in table.find_all('tr'):
            cells = []
            for td in tr.find_all(['td', 'th']):
                text = td.get_text().strip()
                # Clean up text for LaTeX
                text = text.replace('%', '\\%')
                text = text.replace('_', '\\_')
                text = text.replace('#', '\\#')
                cells.append(text)
            if cells:
                rows.append(' & '.join(cells) + ' \\\\')

        if rows:
            ncols = len(rows[0].split(' & ')) if rows else 1
            col_spec = 'l' * ncols

            latex_table = f"""
\\begin{{table}}[htbp]
\\centering
\\caption{{Experimental Results Table {i+1}}}
\\label{{tab:results_{i+1}}}
\\caption{{{caption_prefix} {i+1}}}
\\label{{{label_prefix}_{i+1}}}
\\begin{{tabular}}{{{col_spec}}}
\\toprule
{chr(10).join(rows[:1])}
\\midrule
{chr(10).join(rows[1:])}
\\bottomrule
\\end{{tabular}}
\\end{{table}}
"""
            latex_content.append(latex_table)

    # Save tables
    with open(output_file, 'w') as f:
        f.write('\n'.join(latex_content))

def main():
    parser = argparse.ArgumentParser(description='Convert HTML tables to LaTeX format')
    parser.add_argument('input_file', help='Input HTML file to convert')
    parser.add_argument('-o', '--output', default='tables.tex', 
                       help='Output LaTeX file (default: tables.tex)')
    parser.add_argument('--caption-prefix', default='Experimental Results Table',
                       help='Prefix for table captions (default: "Experimental Results Table")')
    parser.add_argument('--label-prefix', default='tab:results',
                       help='Prefix for table labels (default: "tab:results")')
    parser.add_argument('--column-spec', default='l',
                       help='LaTeX column specification per column (default: "l")')
    
    args = parser.parse_args()
    
    try:
        clean_html_to_latex(args.input_file, args.output, args.caption_prefix, args.label_prefix)
        print(f"HTML tables converted to LaTeX: {args.output}")
    except FileNotFoundError:
        print(f"Error: Input file '{args.input_file}' not found", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"HTML conversion failed: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()