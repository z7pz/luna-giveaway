/*
  Warnings:

  - You are about to drop the column `winners` on the `Giveaway` table. All the data in the column will be lost.
  - You are about to drop the `_GiveawayToUser` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `winners_count` to the `Giveaway` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "_GiveawayToUser" DROP CONSTRAINT "_GiveawayToUser_A_fkey";

-- DropForeignKey
ALTER TABLE "_GiveawayToUser" DROP CONSTRAINT "_GiveawayToUser_B_fkey";

-- AlterTable
ALTER TABLE "Giveaway" DROP COLUMN "winners",
ADD COLUMN     "winners_count" INTEGER NOT NULL;

-- DropTable
DROP TABLE "_GiveawayToUser";

-- CreateTable
CREATE TABLE "_giveaways" (
    "A" BIGINT NOT NULL,
    "B" BIGINT NOT NULL
);

-- CreateTable
CREATE TABLE "_winnings" (
    "A" BIGINT NOT NULL,
    "B" BIGINT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "_giveaways_AB_unique" ON "_giveaways"("A", "B");

-- CreateIndex
CREATE INDEX "_giveaways_B_index" ON "_giveaways"("B");

-- CreateIndex
CREATE UNIQUE INDEX "_winnings_AB_unique" ON "_winnings"("A", "B");

-- CreateIndex
CREATE INDEX "_winnings_B_index" ON "_winnings"("B");

-- AddForeignKey
ALTER TABLE "_giveaways" ADD CONSTRAINT "_giveaways_A_fkey" FOREIGN KEY ("A") REFERENCES "Giveaway"("message_id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_giveaways" ADD CONSTRAINT "_giveaways_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_winnings" ADD CONSTRAINT "_winnings_A_fkey" FOREIGN KEY ("A") REFERENCES "Giveaway"("message_id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_winnings" ADD CONSTRAINT "_winnings_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;
