FROM public.ecr.aws/lambda/python:3.13

# 作業ディレクトリを指定（/var/taskがLambdaのデフォルト）
WORKDIR /var/task

# app.py, requirements.txt をコピー
COPY app.py requirements.txt ./
COPY shared/ ./shared/
# 依存パッケージをインストール
RUN python3.13 -m pip install -r requirements.txt -t .

# Lambdaのエントリーポイントを指定
CMD ["app.lambda_handler"]