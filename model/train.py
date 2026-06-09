import sqlite3
import pandas as pd
import numpy as np
import xgboost as xgb
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_absolute_error
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
import onnx
import os

DB_PATH = "../data/test.db"
MODEL_PATH = "model.onnx"

FEATURES = [
    "was_home", "starts",
    "avg_fpl_last3", "avg_minutes_last3", "avg_goals_last3", "avg_assists_last3",
    "avg_clean_sheets_last3", "avg_saves_last3", "avg_bonus_last3", "avg_ict_last3", "avg_xgi_last3",
    "avg_fpl_last5", "avg_minutes_last5", "avg_goals_last5", "avg_assists_last5",
    "avg_clean_sheets_last5", "avg_saves_last5", "avg_bonus_last5", "avg_ict_last5", "avg_xgi_last5",
    "avg_fpl_season", "avg_minutes_season", "avg_goals_season", "avg_assists_season",
    "avg_clean_sheets_season", "avg_saves_season", "avg_bonus_season", "avg_ict_season", "avg_xgi_season",
    "rest_days", "games_played"
]

TARGET = "total_points"


def load_data():
    conn = sqlite3.connect(DB_PATH)
    df = pd.read_sql_query("SELECT * FROM gameweek_stats", conn)
    conn.close()
    return df


def compute_sample_weights(df):
    max_round = df["round"].max()
    weights = df["round"].apply(lambda r: 1.0 if r >= max_round - 10 else 0.5 if r >= max_round - 20 else 0.25)
    return weights.values


def train(df):
    X = df[FEATURES].astype(float).values
    y = df[TARGET].astype(float)
    weights = compute_sample_weights(df)

    X_train, X_test, y_train, y_test, w_train, _ = train_test_split(
        X, y, weights, test_size=0.2, random_state=42
    )

    model = xgb.XGBRegressor(
        n_estimators=300,
        max_depth=6,
        learning_rate=0.05,
        subsample=0.8,
        colsample_bytree=0.8,
        random_state=42
    )

    model.fit(X_train, y_train, sample_weight=w_train)

    preds = model.predict(X_test)
    mae = mean_absolute_error(y_test, preds)
    print(f"Model MAE: {mae:.4f}")

    return model, mae, X_test


def get_current_mae():
    if not os.path.exists("mae.txt"):
        return float("inf")
    with open("mae.txt", "r") as f:
        return float(f.read().strip())


def save_mae(mae):
    with open("mae.txt", "w") as f:
        f.write(str(mae))


def export_to_onnx(model, n_features):
    import onnxmltools
    from onnxmltools.convert import convert_xgboost
    from onnxmltools.convert.common.data_types import FloatTensorType

    initial_type = [("float_input", FloatTensorType([None, n_features]))]
    onnx_model = convert_xgboost(model, initial_types=initial_type)
    with open(MODEL_PATH, "wb") as f:
        f.write(onnx_model.SerializeToString())
    print(f"Model saved to {MODEL_PATH}")


def run():
    print("Loading data...")
    df = load_data()
    print(f"Loaded {len(df)} rows")

    print("Training model...")
    model, new_mae, X_test = train(df)

    current_mae = get_current_mae()
    print(f"Current MAE: {current_mae:.4f} | New MAE: {new_mae:.4f}")

    if new_mae < current_mae:
        print("New model is better — deploying...")
        export_to_onnx(model, len(FEATURES))
        save_mae(new_mae)
    else:
        print("Current model is better — keeping existing model.")


if __name__ == "__main__":
    run()