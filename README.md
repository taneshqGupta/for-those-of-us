### Skill-Swap: Learn, Teach, Socialise

---

#### Q: How to set it up and get it running?
###### Ans: No Set-Up required at all!
######      Just go to the either of the following links -

###### [skillswap.taneshq.me](https://skillswap.taneshq.me)
###### [skillswap.taneshq.iitmandi.in.net](https://skillswap.taneshq.iitmandi.in.net)

---
#### Q: What social problem does _Skill-Swap_ Address?
###### Ans: In times like ours when upskilling always has a cost attached to it, regardless of domain. _Skill-Swap_ offers a  way to share your skills within your local community and get to form social bonds and learn something in return.
###### Intended User-Base: People of all ages, especially those new to any community, be it a city, or a village. People who struggle making friends. People who wish to learn something new & Those who love to teach.


---
#### Key Features:

---

##### Real-Time Map [Based on Indian Pin Codes]

<img src="/screenshots/map-2025-09-05_02.14.48-ezgif.com-video-to-gif-converter.gif">

---

##### Responsiveness (Compatibility with screens of all sizes, phones, tablets, monitors..)

###### This App is well optimised to look and work well on mobile phones, as a standalone application installed as a PWA (Progressive Web-App). I am not quite sure if the PWA functionality works well on iOS or Mac yet, but I am certain it works well on Linux, Windows, and Android.

<img src="/screenshots/responsive-2025-09-05_01.13.39-ezgif.com-optimize.gif">

---

##### Dynamic + Persistant Theme-Switching (uses browser cookies to store user's choice of theme)
<img src="/screenshots/themespp-2025-09-05_01.24.49-ezgif.com-optimize.gif">

---


##### Changing Profile Picture
<img src="/screenshots/pfpppp-2025-09-05_01.45.31-ezgif.com-video-to-gif-converter.gif">

---


##### Install-App Button that triggers your browser to let you install the PWA (Progressive Web-App)
<img src="/screenshots/install.png">



---

##### Edit/Delete functionality for your Skill Offers/Requests
<img src="/screenshots/edit.png">

---

##### Tech-Stack:
###### Rust-lang for Backend, Svelte-Kit for SSR Frontend, Postgres as Database, Cloudinary for compression and handling of profile-pictures, Tailwind-CSS & DaisyUI for styling, Leaflet for Map

###### I thank those who maintain these languages, frameworks, and services as without these, this side project of mine would never have come to fruition, or atleast, not as smoothly as it has.

---


##### Personal Side Note

###### But ofcourse, this is still just a minimum viable product, any such IRL consequences based platform must be implemented with heavy security, verification, and blocking features. I would love to implement such features in the future, but as of now, this is just a proof of concept, just an MVP.

###### Although I haven't implemented features like email-verification, IP logging, user-blocking, and captchas yet; I have implemented password-hashing in rust so any password you make at time of account creation gets hashed, and not even the creator (me) with access to database, can read your passwords.

###### The code being fully open-source, any one can go and see exactly how passwords hashing is implemented and passwords are stored, which makes your account password completely safe with _Skill-Swap_

---

#### License

###### This project is open-sourced under the MIT License.