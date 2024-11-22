import os
import pandas as pd
from pathlib import Path


def load_dictionary(dictionary_file):
    with open(dictionary_file, "r", encoding="utf-8") as file:
        words = set(line.strip().upper() for line in file)
    return words


def count_dictionary_words(text, dictionary):
    count = 0
    text = text.upper()
    for word in dictionary:
        if word in text:
            count += 1
    return count


def process_csv(file_path, dictionary):
    try:
        df = pd.read_csv(file_path)
        # For better performance, sort by score
        df = df.sort_values(by="Score")
        # Add a column for the number of dictionary words found
        df["Dictionary Matches"] = df["Decrypted Text"].apply(
            lambda x: count_dictionary_words(x, dictionary)
        )
        return df
    except FileNotFoundError:
        print(f"Warning: {file_path} not found. Skipping...")
        return pd.DataFrame()


def get_best_decryptions(df):
    if df.empty:
        return pd.DataFrame()

    best_results = []
    grouped = df.groupby("Cipher Text")
    for cipher_text, group in grouped:
        # Get the row with the highest number of dictionary matches and lowest score
        group = group.sort_values(
            by=["Dictionary Matches", "Score"], ascending=[False, True]
        )
        best_row = group.iloc[0]
        best_results.append(
            {
                "Cipher Text": cipher_text,
                "Key": best_row["Key"],
                "Decrypted Text": best_row["Decrypted Text"],
                "Score": best_row["Score"],
                "Dictionary Matches": best_row["Dictionary Matches"],
            }
        )
    return pd.DataFrame(best_results)


def find_best_overall(combined_df):
    if combined_df.empty:
        return pd.DataFrame()

    best_overall = []
    grouped = combined_df.groupby("Cipher Text")
    for cipher_text, group in grouped:
        group = group.sort_values(
            by=["Dictionary Matches", "Score"], ascending=[False, True]
        )
        best_row = group.iloc[0]
        best_overall.append(best_row)
    return pd.DataFrame(best_overall)


def main():
    root_dir = Path(__file__).parent.parent.parent
    input_dir = root_dir / "data" / "input"
    output_dir = root_dir / "data" / "output"

    output_dir.mkdir(parents=True, exist_ok=True)

    dictionary = load_dictionary(input_dir / "dicionario.txt")

    caesar_df = process_csv(root_dir / "data" / "caesar.csv", dictionary)
    substitution_df = process_csv(
        root_dir / "data" / "substitution.csv", dictionary)
    vigenere_df = process_csv(root_dir / "data" / "vigenere.csv", dictionary)

    # Get best decryptions for each
    cipher_results = {
        "Caesar": get_best_decryptions(caesar_df),
        "Substitution": get_best_decryptions(substitution_df),
        "Vigenere": get_best_decryptions(vigenere_df),
    }

    for cipher_type, df in cipher_results.items():
        if not df.empty:
            df["Cipher Type"] = cipher_type

    combined_results = pd.concat(cipher_results.values(), ignore_index=True)

    if combined_results.empty:
        print("No results found. Make sure the cipher analysis has been run first.")
        return

    best_overall = find_best_overall(combined_results)

    print("\nBest Decryptions:")
    print(
        best_overall[
            [
                "Cipher Text",
                "Cipher Type",
                "Key",
                "Decrypted Text",
                "Score",
                "Dictionary Matches",
            ]
        ]
    )

    output_file = output_dir / "best_decryptions.csv"
    best_overall.to_csv(output_file, index=False)
    print(f"\nResults saved to: {output_file}")


if __name__ == "__main__":
    main()
