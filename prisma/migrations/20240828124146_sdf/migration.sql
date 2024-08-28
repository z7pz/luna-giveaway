/*
  Warnings:

  - Added the required column `host` to the `Giveaway` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE "Giveaway" ADD COLUMN     "host" TEXT NOT NULL;
