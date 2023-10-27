---users definition

CREATE TABLE `users` (
  `userId` varchar(50) NOT NULL,
  `name` varchar(50) DEFAULT NULL,
  `role` varchar(20) DEFAULT NULL,
  `enabled` tinyint(1) DEFAULT NULL,
  `createdAt` datetime DEFAULT current_timestamp(),
  `updatedAt` datetime DEFAULT current_timestamp(),
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=26403 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--- table sample entries
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Active Dev', 'Fela Kuti', 'ADMIN', 1, '2022-08-08 22:11:14.000', '2023-04-01 15:17:08.000', 1);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Active Dev2', 'Active2', 'ADMIN', 1, '2022-08-08 22:11:14.000', '2022-08-08 22:11:14.000', 2);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('jane.foster', 'Jane Foster', 'ADMIN', 1, '2022-08-13 00:00:00.000', '2022-08-13 00:00:00.000', 4);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('JOHN.DOE', 'JOHN DOE', 'ADMIN', 1, '2022-08-07 00:00:00.000', '2022-08-07 00:00:00.000', 5);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Funky', '', 'ADMIN', 1, '2022-10-08 14:05:00.000', '2022-10-08 14:05:00.000', 6);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('silly.joe', 'Silly Joe', 'REGULAR', 1, NULL, NULL, 7);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('silly.joe', 'Happy Joshe', 'REGULAR', 1, NULL, '2022-10-16 12:11:34.000', 8);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Tinleigh.Munos', 'Tinleigh Munos', 'REGULAR', 1, '2022-10-09 10:47:30.000', '2022-10-09 10:47:30.000', 9);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('bill.haye', 'Bill Haye', 'REGULAR', 1, '2022-10-09 10:49:27.000', '2022-10-09 10:49:27.000', 10);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Bayla.Waits', 'Bayla Waits', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 11);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kolt.Meador', 'Kolt Meador', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 12);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Lenny.Mccartney', 'Lenny Mccartney', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 13);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kabir.Devries', 'Kabir Devries', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 14);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Vita.Mcelhaney', 'Vita Mcelhaney', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 15);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Raven.Alton', 'Raven Alton', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 16);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Bruno.Painter', 'Bruno Painter', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 17);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Alaric.Chatman', 'Alaric Chatman', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 18);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Melody.Hale', 'Melody Hale', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 19);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Alicia.Solomon', 'Alicia Solomon', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 20);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jaiyana.Hoggard', 'Jaiyana Hoggard', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 21);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Devante.Loving', 'Devante Loving', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 22);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jaelyn.Mckibben', 'Jaelyn Mckibben', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 23);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Juelz.Boudreaux', 'Juelz Boudreaux', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 26);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Darrin.Witmer', 'Darrin Witmer', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 27);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Draya.Conde', 'Draya Conde', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 28);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Shiloh.Saenz', 'Shiloh Saenz', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 29);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Thalia.Colvin', 'Thalia Colvin', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 30);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Tristyn.Ballinger', 'Tristyn Ballinger', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 31);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Arjun.Shoemaker', 'Arjun Shoemaker', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 32);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jania.Higa', 'Jania Higa', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 33);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Amoura.Mauk', 'Amoura Mauk', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 34);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Lillyan.Denham', 'Lillyan Denham', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 35);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Ekam.Fetters', 'Ekam Fetters', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 36);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Aram.Zimmermann', 'Aram Zimmermann', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 37);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Waylon.Reese', 'Waylon Reese', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 38);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Remington.Dempsey', 'Remington Dempsey', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 39);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Nicco.Jimerson', 'Nicco Jimerson', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 40);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Ziyad.Friedrich', 'Ziyad Friedrich', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 41);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Yahel.Barnwell', 'Yahel Barnwell', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 42);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Annabella.Hickman', 'Annabella Hickman', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 43);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Cierra.Mauldin', 'Cierra Mauldin', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 44);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Chizaram.Vanhouten', 'Chizaram Vanhouten', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 45);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Queena.Reichard', 'Queena Reichard', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 46);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Cozette.Stonge', 'Cozette Stonge', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 47);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Gentry.Gaddis', 'Gentry Gaddis', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 48);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Aleksandr.Hollister', 'Aleksandr Hollister', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 49);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Arionna.Slocum', 'Arionna Slocum', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 50);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Neena.Traxler', 'Neena Traxler', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 51);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Janiel.Lees', 'Janiel Lees', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 52);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Azaiah.Barnhill', 'Azaiah Barnhill', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 53);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Daren.Croteau', 'Daren Croteau', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 54);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Nelly.Hamel', 'Nelly Hamel', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 55);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Ellyson.Massengale', 'Ellyson Massengale', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 56);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Armani.Beach', 'Armani Beach', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 57);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Bless.Swinson', 'Bless Swinson', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 58);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Lane.Beattie', 'Lane Beattie', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 59);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Linus.Marroquin', 'Linus Marroquin', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 60);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Ashley.Carroll', 'Ashley Carroll', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 61);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Amiyah.Delacruz', 'Amiyah Delacruz', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 62);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Trinity.Beck', 'Trinity Beck', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 63);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Zaiden.Hendricks', 'Zaiden Hendricks', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 64);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Leigha.Wahl', 'Leigha Wahl', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 65);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Nevada.Noto', 'Nevada Noto', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 66);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Madina.Carmack', 'Madina Carmack', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 67);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Infinity.Mcgaughey', 'Infinity Mcgaughey', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 68);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Justyn.Denis', 'Justyn Denis', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 69);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Balian.Lindholm', 'Balian Lindholm', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 70);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Loren.Thornburg', 'Loren Thornburg', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 71);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Payson.Kozak', 'Payson Kozak', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 72);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Rakan.Look', 'Rakan Look', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 73);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Whitlee.Kile', 'Whitlee Kile', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 74);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kallan.Leonardo', 'Kallan Leonardo', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 75);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Basya.Cavin', 'Basya Cavin', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 76);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Aarnav.Capone', 'Aarnav Capone', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 77);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Rylie.Middleton', 'Rylie Middleton', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 78);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Elijiah.Mcmillion', 'Elijiah Mcmillion', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 79);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Yosef.Bui', 'Yosef Bui', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 80);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Teegan.Griego', 'Teegan Griego', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 81);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Marcelina.Lamp', 'Marcelina Lamp', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 82);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Harland.Stuck', 'Harland Stuck', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 83);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jetta.Arambula', 'Jetta Arambula', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 84);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Amayah.Kimble', 'Amayah Kimble', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 85);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Inaaya.Hutto', 'Inaaya Hutto', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 86);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jadyn.Latimer', 'Jadyn Latimer', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 87);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Aleister.Stamey', 'Aleister Stamey', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 88);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Makenley.Zell', 'Makenley Zell', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 89);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Olivia.Johnson', 'Olivia Johnson', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 90);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Tehila.Harnish', 'Tehila Harnish', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 91);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jadalyn.Lore', 'Jadalyn Lore', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 92);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Derrick.Ho', 'Derrick Ho', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 93);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Liesel.Munro', 'Liesel Munro', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 94);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Yvette.Clancy', 'Yvette Clancy', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 95);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Miking.Ciccone', 'Miking Ciccone', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 96);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Jemma.Greenwood', 'Jemma Greenwood', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 97);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Tamar.Yan', 'Tamar Yan', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 98);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Cortez.Fernandes', 'Cortez Fernandes', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 99);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Julius.Boone', 'Julius Boone', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 100);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Batoul.Chumley', 'Batoul Chumley', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 101);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Yulian.Mulvihill', 'Yulian Mulvihill', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 102);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kenny.Champion', 'Kenny Champion', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 103);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kimani.Stancil', 'Kimani Stancil', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 104);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Veronica.Blankenship', 'Veronica Blankenship', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 105);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Lucius.Harley', 'Lucius Harley', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 106);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Mohamedamin.Ruddy', 'Mohamedamin Ruddy', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 107);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Audie.Blatt', 'Audie Blatt', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 108);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Dailyn.Tighe', 'Dailyn Tighe', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 109);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Clinton.Wilburn', 'Clinton Wilburn', 'REGULAR', 1, '2022-10-09 10:51:54.000', '2022-10-09 10:51:54.000', 110);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Osvaldo.Goodson', 'Osvaldo Goodson', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 111);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Kelly.Good', 'Kelly Good', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 112);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Guillermo.Inman', 'Guillermo Inman', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 113);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Akira.Casillas', 'Akira Casillas', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 114);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Naylah.Mulvey', 'Naylah Mulvey', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 115);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('King.Keller', 'King Keller', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 116);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Adilyn.Mattson', 'Adilyn Mattson', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 117);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Calee.Gasca', 'Calee Gasca', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 118);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Charley.Dejesus', 'Charley Dejesus', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 119);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Taniyah.Box', 'Taniyah Box', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 120);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Mercer.Hannigan', 'Mercer Hannigan', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 121);
INSERT INTO AUTHAPI.users
(userId, name, `role`, enabled, createdAt, updatedAt, id)
VALUES('Raygan.Oster', 'Raygan Oster', 'REGULAR', 1, '2022-10-09 10:54:54.000', '2022-10-09 10:54:54.000', 122);
