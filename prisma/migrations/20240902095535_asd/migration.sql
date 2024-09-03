-- CreateTable
CREATE TABLE "User" (
    "id" BIGINT NOT NULL,

    CONSTRAINT "User_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Oauth" (
    "id" TEXT NOT NULL,
    "access_token" TEXT NOT NULL,
    "refresh_token" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "user_id" BIGINT NOT NULL,

    CONSTRAINT "Oauth_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "_GiveawayToUser" (
    "A" BIGINT NOT NULL,
    "B" BIGINT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Oauth_user_id_key" ON "Oauth"("user_id");

-- CreateIndex
CREATE UNIQUE INDEX "_GiveawayToUser_AB_unique" ON "_GiveawayToUser"("A", "B");

-- CreateIndex
CREATE INDEX "_GiveawayToUser_B_index" ON "_GiveawayToUser"("B");

-- AddForeignKey
ALTER TABLE "Oauth" ADD CONSTRAINT "Oauth_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_GiveawayToUser" ADD CONSTRAINT "_GiveawayToUser_A_fkey" FOREIGN KEY ("A") REFERENCES "Giveaway"("message_id") ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "_GiveawayToUser" ADD CONSTRAINT "_GiveawayToUser_B_fkey" FOREIGN KEY ("B") REFERENCES "User"("id") ON DELETE CASCADE ON UPDATE CASCADE;
