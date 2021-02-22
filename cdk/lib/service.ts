import * as core from "@aws-cdk/core";
import * as apigateway from "@aws-cdk/aws-apigateway";
import * as lambda from "@aws-cdk/aws-lambda";
import * as s3 from "@aws-cdk/aws-s3";

export class Service extends core.Construct {
    constructor(scope: core.Construct, id: string) {
        super(scope, id);

        const bucket = new s3.Bucket(this, "RustOpenAPIBucket");

        const handler = new lambda.Function(this, "RocketFunction", {
            runtime: lambda.Runtime.PROVIDED_AL2,
            code: lambda.Code.fromAsset("../bootstrap"),
            handler: "unused",
            environment: {
                BUCKET: bucket.bucketName
            }
        });

        bucket.grantReadWrite(handler);

        const api = new apigateway.LambdaRestApi(this, "RocketAPI", {
            handler,
            restApiName: "Rocket API",
            description: "An OpenAPI serverless app made with Rust's Rocket framework.",

        });
    }
}