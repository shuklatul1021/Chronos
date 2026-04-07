-- Your SQL goes here
-- CreateEnum
CREATE TYPE "jobtype" AS ENUM ('SEND_EMAIL', 'NOTIFICATION', 'PDF_GENERATOR');

-- CreateTable
CREATE TABLE "user" (
    "id" TEXT NOT NULL,
    "username" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "job" (
    "id" TEXT NOT NULL,
    "jobtitle" TEXT NOT NULL,
    "jobdescription" TEXT,
    "type" "jobtype" NOT NULL,
    "payload" TEXT NOT NULL,
    "runs_at" TEXT,
    "max_retries" INTEGER NOT NULL,
    "webhook_url" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Job_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "User_username_key" ON "user"("username");

-- CreateIndex
CREATE UNIQUE INDEX "User_email_key" ON "user"("email");

-- AddForeignKey
ALTER TABLE "job" ADD CONSTRAINT "Job_userId_fkey" FOREIGN KEY ("userId") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
