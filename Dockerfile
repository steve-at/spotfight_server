# FROM debian:bullseye-slim
# ARG APP=/usr/src/app
# RUN apt-get update \
#     && apt-get install -y ca-certificates tzdata libc6 \
#     && rm -rf /var/lib/apt/lists/*
# ENV TZ=Etc/UTC \
#     APP_USER=appuser
# RUN groupadd $APP_USER \
#     && useradd -g $APP_USER $APP_USER \
#     && mkdir -p ${APP}

    
# COPY ./server ${APP}

# RUN chown -R $APP_USER:$APP_USER ${APP}
# USER $APP_USER
# WORKDIR ${APP}

# EXPOSE 7777/udp
# EXPOSE 7777/tcp

# RUN chmod +x ${APP}/SpotfightServer.sh
# RUN echo "BUILT"
# ENTRYPOINT ["./SpotfightServer.sh"]
# CMD ["./SpotfightServer.sh"]

FROM debian:latest
ARG APP=/usr/src/app
RUN apt-get update \
    && apt-get upgrade -y

COPY ./server ${APP}

# Add a new ue4 user. The dedicated server cannot be run as root.
RUN useradd ue4 \
    && chown ue4:ue4 -R ${APP}
WORKDIR ${APP}

USER ue4
EXPOSE 7777-7787/udp
# EXPOSE 7778/udp

ENTRYPOINT [ "./SpotfightServer.sh" ]