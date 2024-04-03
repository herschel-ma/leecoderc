from __future__ import print_function

import pandas as pd
import numpy as np


def work(col):
    test_step = col
    # merge value of index, which is nan to previous value
    notna = test_step.index.notna().tolist()
    idx = []
    ans = ["" for _ in range(len(notna))]
    pre_v = np.nan
    for i, v in enumerate(notna):
        if v:
            idx.append(i)

    for i in range(len(ans)):
        pre_v = test_step.iloc[i]
        if i in idx:
            ans[i] = pre_v
            for j in range(i + 1, len(ans)):
                if j not in idx:
                    ans[i] += "*" + test_step.iloc[j]
                else:
                    break

    # step_idx = test_step.index.dropna()
    ans_step = pd.DataFrame(data=ans, index=test_step.index)
    return ans_step


def run(path):
    # read sheet1 from xlsx file
    # df = pd.read_excel(path, "Sheet1", index_col=0)
    df = pd.read_excel(path, "数据融合校正系统", index_col=0)
    # read L col and M col from sheet1
    test_step = df["测试步骤（必填）"]
    df["测试步骤（必填）"] = work(test_step)
    test_method = df["预期结果（必填）"]
    df["预期结果（必填）"] = work(test_method)
    df = df.dropna(axis=0, how="all")
    write(df)
    df = pd.read_excel("./test_ans.xlsx", "数据融合校正系统", index_col=0)
    df = df.dropna(axis=0, how="all")
    write(df)


def write(df):
    df.to_excel("./test_ans.xlsx", sheet_name="Sheet1")


if __name__ == "__main__":
    run("./test.xlsx")
