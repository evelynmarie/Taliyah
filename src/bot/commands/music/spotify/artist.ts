/**
 * artist.ts -- Retrieves information about an artist on Spotify.
 *
 * Copyright (c) 2019-present Kamran Mackey.
 *
 * Ellie is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Ellie is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ellie. If not, see <https://www.gnu.org/licenses/>.
 */

import * as request from 'superagent';

import { Command } from 'discord-akairo';
import { Message, MessageEmbed } from 'discord.js';

import { Util } from '../../../utils/Util';

import stripHtml from 'string-strip-html';

export default class SpotifyArtistCommand extends Command {
  public constructor() {
    super('spotify-artist', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays information about an artist on Spotify.',
        usage: '<aritst>',
      },
      args: [
        {
          id: 'artist',
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { artist }: { artist: string }) {
    const errEmbed = new MessageEmbed();

    if (!artist) {
      errEmbed.setColor(0x1DB954);
      errEmbed.setTitle('Error: No artist name provided.');
      errEmbed.setDescription('You didn\'t provide an artist name. Please provide one and then '
        + 'try again!',
      );

      return message.channel.send(errEmbed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      // I really hate having to do this, but because Spotify for some unknown reason
      // decided to not expose artist biographies in their public API, I have to sadly
      // resort to using their backend API used with their app clients and web player
      // to be able to retrieve artist biographies. I really hope this will only be a
      // short-term solution, but honestly you never know with Spotify.
      const spotifyBackendUrl = 'https://spclient.wg.spotify.com/open-backend-2/v1';
      const spotifyBackendEndpoint = '/artists/';
      const spotifyBackendFullUrl = spotifyBackendUrl + spotifyBackendEndpoint;

      this.client.spotify.searchArtists(artist, { limit: 1, offset: 0 }, (err, res) => {
        const artistId = res.body.artists.items[0].id;

        this.client.spotify.getArtist(artistId).then(async (res) => {
          const artistName = res.body.name;
          const artistGenres = res.body.genres.join(', ');
          const artistFollowers = res.body.followers.total;
          const artistLink = res.body.external_urls.spotify;
          const artistImage = res.body.images[0].url;

          const aboutRequest = await request.get(spotifyBackendFullUrl + artistId).set({
            // TODO: Make this automatically refresh somehow as having to put a new token
            //       in the configuration file every hour is quite a pain in the butt and
            //       will honestly get quite annoying after a while.
            Authorization: 'Bearer ' + this.client.config.spotify.wgAccessToken,
            // Use the Chrome user agent to identify the bot as just a normal browser that's
            // accessing the private API.
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3748.0 Safari/537.36',
          });

          let artistBiography: string;

          if (aboutRequest.body.hasOwnProperty('bio')) {
            artistBiography = stripHtml(Util.shorten(aboutRequest.body.bio, 600));
          } else {
            artistBiography = 'No biography :(';
          }

          const artistEmbed = new MessageEmbed();
          artistEmbed.setTitle(artistName);
          artistEmbed.setColor(0x1DB954);
          artistEmbed.setURL(artistLink);
          artistEmbed.setThumbnail(artistImage);
          artistEmbed.setDescription(
            `${artistBiography}\n\n` +
            `**Followers**: ${artistFollowers}\n` +
            `**Monthly Listeners**: ${aboutRequest.body.artistInsights.monthly_listeners}\n` +
            `**Genres**: ${artistGenres ? artistGenres : 'No genres available.'}`,
          );
          artistEmbed.setFooter(`Embed length: ${artistEmbed.length}`);

          return message.channel.send(artistEmbed);
        });
      });
    });
  }
}
