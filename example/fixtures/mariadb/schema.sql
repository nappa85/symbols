
DROP TABLE IF EXISTS `best_selling_video_games`;

CREATE TABLE `best_selling_video_games` (
  `rank` tinyint(2) NOT NULL,
  `name` varchar(255) NOT NULL,
  `sales` varchar(255) NOT NULL,
  `series` varchar(255) NOT NULL,
  `platforms` varchar(255) NOT NULL,
  `initial_release_date` varchar(255) NOT NULL,
  `developer` varchar(255) NOT NULL,
  `publisher` varchar(255) NOT NULL,
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- list taken from https://en.wikipedia.org/wiki/List_of_best-selling_video_games
-- normalized with regexp ^(\d+)\s+\t([^\t]+)\s+\t([\d,]+)\s+\t([^\t]+)\s+\t([^\t]+)\s+\t([^\t]+)\s+\t([^\t]+)\s+\t([^\t]+)\s+\t$
INSERT INTO `best_selling_video_games` VALUES
(1, 'Minecraft', '238,000,000', 'Minecraft', 'Multi-platform', 'November 18, 2011', 'Mojang Studios', 'Xbox Game Studios'),
(2, 'Grand Theft Auto V', '160,000,000', 'Grand Theft Auto', 'Multi-platform', 'September 17, 2013', 'Rockstar North', 'Rockstar Games'),
(3, 'Tetris (EA)', '100,000,000', 'Tetris', 'Multi-platform', 'September 12, 2006', 'EA Mobile', 'Electronic Arts'),
(4, 'Wii Sports', '82,900,000', 'Wii', 'Wii', 'November 19, 2006', 'Nintendo EAD', 'Nintendo'),
(5, 'PUBG: Battlegrounds', '75,000,000', 'PUBG Universe', 'Multi-platform', 'December 20, 2017', 'PUBG Corporation', 'PUBG Corporation'),
(6, 'Super Mario Bros.', '58,000,000', 'Super Mario', 'Multi-platform', 'September 13, 1985', 'Nintendo R&D4', 'Nintendo'),
(7, 'Mario Kart 8 / Deluxe', '51,810,000', 'Mario Kart', 'Wii U / Switch', 'May 29, 2014', 'Nintendo EAD', 'Nintendo'),
(8, 'Pokemon Red / Green / Blue / Yellow', '47,520,000', 'Pokemon', 'Game Boy / Color', 'February 27, 1996', 'Game Freak', 'Nintendo'),
(9, 'Terraria', '44,000,000', 'None', 'Multi-platform', 'May 16, 2011', 'Re-Logic', 'Re-Logic / 505 Games'),
(10, 'Wii Fit / Plus', '43,800,000', 'Wii', 'Wii', 'December 1, 2007', 'Nintendo EAD', 'Nintendo'),
(11, 'Red Dead Redemption 2', '43,000,000', 'Red Dead', 'Multi-platform', 'October 26, 2018', 'Rockstar Studios', 'Rockstar Games'),
(11, 'Tetris (Nintendo)', '43,000,000', 'Tetris', 'Game Boy / NES', 'June 14, 1989', 'Nintendo R&D1', 'Nintendo'),
(13, 'Pac-Man', '42,071,635', 'Pac-Man', 'Multi-platform', 'May 22, 1980', 'Namco', 'Namco'),
(14, 'The Witcher 3 / Hearts of Stone / Blood and Wine', '40,000,000', 'The Witcher', 'Multi-platform', 'May 19, 2015', 'CD Projekt Red', 'CD Projekt'),
(15, 'Animal Crossing: New Horizons', '37,620,000', 'Animal Crossing', 'Nintendo Switch', 'March 20, 2020', 'Nintendo EPD', 'Nintendo'),
(16, 'Mario Kart Wii', '37,380,000', 'Mario Kart', 'Wii', 'April 10, 2008', 'Nintendo EAD', 'Nintendo'),
(17, 'Wii Sports Resort', '33,140,000', 'Wii', 'Wii', 'June 25, 2009', 'Nintendo EAD', 'Nintendo'),
(18, 'New Super Mario Bros.', '30,800,000', 'Super Mario', 'Nintendo DS', 'May 15, 2006', 'Nintendo EAD', 'Nintendo'),
(19, 'New Super Mario Bros. Wii', '30,320,000', 'Super Mario', 'Wii', 'November 11, 2009', 'Nintendo EAD', 'Nintendo'),
(20, 'The Elder Scrolls V: Skyrim', '30,000,000', 'The Elder Scrolls', 'Multi-platform', 'November 11, 2011', 'Bethesda Game Studios', 'Bethesda Softworks'),
(20, 'Call of Duty: Modern Warfare', '30,000,000', 'Call of Duty', 'Multi-platform', 'October 25, 2019', 'Infinity Ward', 'Activision'),
(20, 'Diablo III / Reaper of Souls', '30,000,000', 'Diablo', 'Multi-platform', 'May 16, 2012', 'Blizzard Entertainment', 'Blizzard Entertainment'),
(20, 'Human: Fall Flat', '30,000,000', 'None', 'Multi-platform', 'July 22, 2016', 'No Brakes Games', 'Curve Digital'),
(24, 'Pokemon Gold / Silver / Crystal', '29,490,000', 'Pokemon', 'Game Boy Color', 'November 21, 1999', 'Game Freak', 'Nintendo'),
(25, 'Duck Hunt', '28,300,000', 'None', 'NES', 'April 21, 1984', 'Nintendo R&D1', 'Nintendo'),
(26, 'Wii Play', '28,020,000', 'Wii', 'Wii', 'December 2, 2006', 'Nintendo EAD', 'Nintendo'),
(27, 'Grand Theft Auto: San Andreas', '27,500,000', 'Grand Theft Auto', 'Multi-platform', 'October 26, 2004', 'Rockstar North', 'Rockstar Games'),
(28, 'The Legend of Zelda: Breath of the Wild', '27,490,000', 'The Legend of Zelda', 'Wii U / Switch', 'March 3, 2017', 'Nintendo EPD', 'Nintendo'),
(29, 'Super Smash Bros. Ultimate', '27,400,000', 'Super Smash Bros.', 'Nintendo Switch', 'December 7, 2018', 'Bandai Namco Studios / Sora Ltd.', 'Nintendo'),
(30, 'Super Mario World', '26,662,500', 'Super Mario', 'Multi-platform', 'November 21, 1990', 'Nintendo EAD', 'Nintendo'),
(31, 'Call of Duty: Modern Warfare 3', '26,500,000', 'Call of Duty', 'Multi-platform', 'November 8, 2011', 'Infinity Ward / Sledgehammer', 'Activision'),
(32, 'Call of Duty: Black Ops', '26,200,000', 'Call of Duty', 'Multi-platform', 'November 9, 2010', 'Treyarch', 'Activision'),
(33, 'Borderlands 2', '26,000,000', 'Borderlands', 'Multi-platform', 'September 18, 2012', 'Gearbox Software', '2K Games'),
(34, 'Pokemon Sun / Moon / Ultra Sun / Ultra Moon', '25,310,000', 'Pokemon', 'Nintendo 3DS', 'November 18, 2016', 'Game Freak', 'Nintendo / The Pokemon Company'),
(35, 'Grand Theft Auto IV', '25,000,000', 'Grand Theft Auto', 'Multi-platform', 'April 29, 2008', 'Rockstar North', 'Rockstar Games'),
(36, 'Pokemon Diamond / Pearl / Platinum', '24,730,000', 'Pokemon', 'Nintendo DS', 'September 28, 2006', 'Game Freak', 'Nintendo / The Pokemon Company'),
(37, 'Super Mario Bros. 3', '24,430,000', 'Super Mario', 'Multi-platform', 'October 23, 1988', 'Nintendo EAD', 'Nintendo'),
(38, 'Call of Duty: Black Ops II', '24,200,000', 'Call of Duty', 'Multi-platform', 'November 12, 2012', 'Treyarch', 'Activision'),
(39, 'Kinect Adventures!', '24,000,000', 'None', 'Xbox 360', 'November 4, 2010', 'Good Science Studio', 'Xbox Game Studios'),
(39, 'FIFA 18', '24,000,000', 'FIFA', 'Multi-platform', 'September 29, 2017', 'EA Vancouver', 'EA Sports'),
(41, 'Sonic the Hedgehog', '23,982,960', 'Sonic the Hedgehog', 'Multi-platform', 'June 23, 1991', 'Sonic Team', 'Sega'),
(42, 'Nintendogs', '23,960,000', 'None', 'Nintendo DS', 'April 21, 2005', 'Nintendo EAD', 'Nintendo'),
(43, 'Pokemon Sword / Shield', '23,900,000', 'Pokemon', 'Nintendo Switch', 'November 15, 2019', 'Game Freak', 'Nintendo / The Pokemon Company'),
(44, 'Mario Kart DS', '23,600,000', 'Mario Kart', 'Nintendo DS', 'November 14, 2005', 'Nintendo EAD', 'Nintendo'),
(45, 'Super Mario Odyssey', '23,020,000', 'Super Mario', 'Nintendo Switch', 'October 27, 2017', 'Nintendo EPD', 'Nintendo'),
(46, 'Red Dead Redemption', '23,000,000', 'Red Dead', 'PS3 / Xbox 360', 'May 18, 2010', 'Rockstar San Diego', 'Rockstar Games'),
(47, 'Super Mario 64 / DS', '22,960,000', 'Super Mario', 'Nintendo 64 / DS', 'June 23, 1996', 'Nintendo EAD', 'Nintendo'),
(48, 'Call of Duty: Modern Warfare 2', '22,700,000', 'Call of Duty', 'Multi-platform', 'November 10, 2009', 'Infinity Ward', 'Activision'),
(49, 'Pokemon Ruby / Sapphire / Emerald', '22,540,000', 'Pokemon', 'Game Boy Advance', 'November 21, 2002', 'Game Freak', 'Nintendo / The Pokemon Company'),
(50, 'New Super Mario Bros. U / Deluxe / Luigi U', '21,600,000', 'Super Mario', 'Wii U / Nintendo Switch', 'November 18, 2012', 'Nintendo EAD', 'Nintendo');
