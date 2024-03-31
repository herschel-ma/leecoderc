from __future__ import print_function

import pandas as pd
import numpy as np


def work(col):
    test_step = col
    # merge value of index, which is nan to previous value
    notna = test_step.index.notna()
    ans = list()
    pre_v = np.nan
    for i in range(len(notna.tolist())):
        if notna[i]:
            pre_v = test_step.iloc[i]
            ans.append(pre_v)
        else:
            ans[-1] += "*" + test_step.iloc[i]

    step_idx = test_step.index.dropna()
    ans_step = pd.DataFrame(data=ans, index=step_idx)
    return ans_step


def run(path):
    # read sheet1 from xlsx file
    df = pd.read_excel(path, "Sheet1", index_col=0)
    # read L col and M col from sheet1
    test_step = df["测试步骤（必填）"]
    df["测试步骤（必填）"] = work(test_step)

    test_method = df["预期结果（必填）"]
    df["预期结果（必填）"] = work(test_method)
    df = df.dropna(axis=0, how="all")
    write(df)


def write(df):
    df.to_excel("./test_ans.xlsx", sheet_name="Sheet1")


if __name__ == "__main__":
    run("./test.xlsx")
