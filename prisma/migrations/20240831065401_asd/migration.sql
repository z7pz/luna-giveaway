/*
  Warnings:

  - Added the required column `type` to the `EmbedSettings` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "EmbedSettings" ADD COLUMN     "type" "EmbedType" NOT NULL;
